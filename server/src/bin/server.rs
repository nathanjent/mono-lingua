extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate core;

use actix::*;
use actix_web::*;
use std::env;

use core::*;

/// 404 handler
fn p404(_req: HttpRequest) -> Result<HttpResponse> {
    // html
    let html = r#"<!DOCTYPE html>
    <html>
    <head>
    <title>actix - basics</title>
    </head>
    <body>
    <a href="index.html">back to home</a>
    <h1>404</h1>
</body>
</html>"#;

    // response
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(html)
        .unwrap())
}

/// This handler uses `HttpRequest::json()` for loading serde json object.
fn index(req: HttpRequest) -> Result<HttpResponse> {
    ws::start(req, WsMessage {
        uuid: "BEEF".into(),
        data: SharedObj {
            name: "blah".into(),
            message: "".into(),
        },
    })
}

/// Define http actor
#[derive(Debug, Serialize, Deserialize)]
struct WsMessage {
    uuid: String,
    data: SharedObj,
}

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
                    
                        let out_val = WsMessage {
                            uuid: val.uuid,
                            data: SharedObj {
                                name: val.data.name,
                                message: val.data.message,
                            },
                        };
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

    let sys = actix::System::new("ws-example");

    HttpServer::new(|| {
        Application::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // Websocket
            .resource("/ws/", |r| r.route().f(index))
            .handler("/dist", fs::StaticFiles::new("dist/", false).index_file("index.html"))
            .handler("/", fs::StaticFiles::new("dist/", false).index_file("index.html"))
            .default_resource(|r| {
                r.method(Method::GET).f(p404);
                r.route().p(pred::Not(pred::Get())).f(|_req| httpcodes::HTTPMethodNotAllowed);
            })
    }).bind("127.0.0.1:8081")
        .expect("Can not bind to 127.0.0.1:8081")
        .start();

    let _ = sys.run();
}
