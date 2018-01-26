extern crate actix_web;
extern crate env_logger;

use actix_web::*;
use std::{io, env};

/// 404 handler
fn p404(req: HttpRequest) -> Result<HttpResponse> {

    // html
    let html = r#"<!DOCTYPE html><html><head><title>actix - basics</title><link rel="shortcut icon" type="image/x-icon" href="/favicon.ico" /></head>
<body>
    <a href="index.html">back to home</a>
    <h1>404</h1>
</body>
</html>"#;

    // response
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(html).unwrap())
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(
        || {
            Application::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // cookie session middleware
            .middleware(middleware::SessionStorage::new(
                    middleware::CookieSessionBackend::build(&[0; 32])
                    .secure(false)
                    .finish()))
            .resource("/", |r| r.method(Method::GET).f(|req| {
                println!("{:?}", req);

                HttpResponse::Found()
                    .header("LOCATION", "/index.html")
                    .finish()
            }))
            .handler("/", fs::StaticFiles::new("dist/", true))
            .default_resource(|r| {
                r.method(Method::GET).f(p404);
                r.route().p(pred::Not(pred::Get())).f(|req| httpcodes::HTTPMethodNotAllowed);
            })
        })
        .bind("127.0.0.1:8081").expect("Can not bind to 127.0.0.1:8081")
        .run();
}
