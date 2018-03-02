extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix::*;
use actix_web::*;
use std::env;

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

/// do websocket handshake and start `Ws` actor
fn ws_index(req: HttpRequest) -> Result<HttpResponse> {
    ws::start(req, Ws)
}

/// Define http actor
struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

/// Define Handler for ws::Message message
impl StreamHandler<ws::Message, ws::WsError> for Ws {

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
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
            .resource("/ws/", |r| r.route().f(ws_index))
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
