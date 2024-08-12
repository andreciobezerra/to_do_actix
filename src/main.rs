use actix_web::{App, HttpServer};
use web_app::views::views_factory;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(views_factory))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
