use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::{thread, time};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello {name}")
}

async fn do_something(_req: HttpRequest) -> impl Responder {
    // println!("number {} is running", number);
    let two_seconds = time::Duration::new(10, 0);
    thread::sleep(two_seconds);

    format!("666")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
            .route("/hell/number", web::get().to(do_something))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
