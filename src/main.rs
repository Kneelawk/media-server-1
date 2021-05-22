mod config;

use crate::config::Config;
use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = match Config::load() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Error loading config file: {}", err);
            return Ok(());
        }
    };

    let base_dir = config.base_dir.clone();

    let mut server = HttpServer::new(move || {
        let base_dir = base_dir.clone();

        App::new().service(Files::new("/files", base_dir).show_files_listing())
    });

    for binding in config.bindings.iter() {
        server = server.bind(binding.clone())?;
    }

    server.run().await
}
