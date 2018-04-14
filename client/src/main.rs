#[macro_use]
extern crate stdweb;
extern crate urlparse;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate core;

use std::rc::Rc;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{Date, document, HtmlElement, WebSocket, window};
use stdweb::web::event::{KeyPressEvent, SocketCloseEvent, SocketErrorEvent, SocketMessageEvent,
                         SocketOpenEvent};
use stdweb::web::html_element::InputElement;

use urlparse::Url;

use core::*;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct WsMessage(SharedObj);

js_deserializable!( WsMessage );

fn main() {
    stdweb::initialize();

    let output_div: HtmlElement = document()
        .query_selector(".output")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let output_msg = Rc::new(move |msg: &str| {
        let elem = document().create_element("p").unwrap();
        elem.set_text_content(msg);
        if let Some(child) = output_div.first_child() {
            output_div.insert_before(&elem, &child).unwrap();
        } else {
            output_div.append_child(&elem);
        }
    });

    output_msg("> Connecting...");
    let url = window().location().unwrap().href().unwrap();
    let url = Url::parse(url);
    let host = url.hostname.unwrap();
    let port = url.port.unwrap();

    let ws = WebSocket::new(&format!("ws://{}:{}/ws/", host, port)).unwrap();

    ws.add_event_listener(enclose!( (output_msg) move |_: SocketOpenEvent| {
        output_msg("> Opened connection");
    }));

    ws.add_event_listener(enclose!( (output_msg) move |_: SocketErrorEvent| {
        output_msg("> Connection Errored");
    }));

    ws.add_event_listener(enclose!( (output_msg) move |event: SocketCloseEvent| {
        output_msg(&format!("> Connection Closed: {}", event.reason()));
    }));

    ws.add_event_listener(enclose!( (output_msg) move |event: SocketMessageEvent| {
        let msg = event.data().into_text().unwrap();
        let val: WsMessage = serde_json::from_str(&*msg).unwrap();
        if let Some(timestamp) = val.0.timestamp {
            let date = Date::from_iso8601(&timestamp);
            output_msg(&format!("{:02}:{:02} : {}", date.get_hours(), date.get_minutes(), val.0.message));
        } else {
            output_msg(&format!("{}", val.0.message));
        }
    }));

    let text_entry: InputElement = document()
        .query_selector(".form input")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    text_entry.add_event_listener(enclose!( (text_entry) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let text: String = text_entry.raw_value();
            if !text.is_empty() {
                text_entry.set_raw_value("");

                let val = WsMessage(SharedObj {
                    timestamp: None,
                    message: text.to_string(),
                });

                let out_val = serde_json::to_string(&val).unwrap();
                js! {
                    console.log(@{&out_val});
                };
                ws.send_text(&out_val).map_err(|_e| {
                    stdweb::private::ConversionError::Custom("WebSocket".to_string())
                });
            }
       }
    }));

    stdweb::event_loop();
}
