use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(Files::new("/files", "/media/jedidiah/anime")))
        .bind("127.0.0.1:9090")?
        .run()
        .await
}
