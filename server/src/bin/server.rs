extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate chrono;

extern crate core;

use actix::*;
use actix_web::*;
use std::env;
use chrono::prelude::*;

use core::*;

/// This handler uses `HttpRequest::json()` for loading serde json object.
fn index(req: HttpRequest) -> Result<HttpResponse> {
    ws::start(req, WsMessage(SharedObj {
        timestamp: None,
        message: "".into(),
    }))
}

/// Define http actor
#[derive(Debug, Serialize, Deserialize)]
struct WsMessage(SharedObj);

impl Actor for WsMessage {
    type Context = ws::WebsocketContext<Self>;
}

/// Define Handler for ws::Message message
impl StreamHandler<ws::Message, ws::WsError> for WsMessage {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WS incoming: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                let _ = serde_json::from_str(&text)
                    .and_then(|val: WsMessage| {
                        let timestamp = Utc::now().to_rfc3339();
                        let out_val = WsMessage(SharedObj {
                            timestamp: Some(timestamp),
                            message: val.0.message,
                        });
                        println!("WS outgoing: {:?}", out_val);
                        ctx.text(serde_json::to_string(&out_val).unwrap());
                        Ok(())
                    });
            }
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("mono-lingua-example");

    HttpServer::new(|| vec![
        // Websocket
        Application::new()
            .prefix("/ws")
            .resource("/", |r| r.route().f(index)),
        Application::new()
            .middleware(middleware::Logger::default())
            .handler("/", fs::StaticFiles::new("dist/", false).index_file("index.html"))
    ]).bind("127.0.0.1:8081")
        .expect("Can not bind to 127.0.0.1:8081")
        .start();

    let _ = sys.run();
}
