extern crate actix_web;
extern crate env_logger;

use actix_web::*;
use std::env;
use std::path::PathBuf;

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
            .handler("/dist", fs::StaticFiles::new("dist/", false))
            .handler("/", fs::StaticFiles::new("dist/", false))
            .default_resource(|r| {
                r.method(Method::GET).f(p404);
                r.route().p(pred::Not(pred::Get())).f(|_req| httpcodes::HTTPMethodNotAllowed);
            })
        })
        .bind("127.0.0.1:8081").expect("Can not bind to 127.0.0.1:8081")
        .run();
}
