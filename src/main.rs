#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod cdn;
mod config;
mod error;
mod logging;
mod util;

use crate::{
    config::Config,
    error::{Result, ResultExt},
};
use actix_web::{middleware::DefaultHeaders, App, HttpServer};
use std::process::exit;

async fn run() -> Result<()> {
    let config = Config::load()?;

    util::ffmpeg::init_ffmpeg()?;

    let server_config = config.clone();
    let mut server = HttpServer::new(move || {
        let config = server_config.clone();

        #[allow(unused_mut)]
        let mut app = App::new();

        // allows CORS from development server to api server
        #[cfg(debug_assertions)]
        let mut app = app.wrap(
            DefaultHeaders::new().header("Access-Control-Allow-Origin", "http://localhost:4200"),
        );

        // app = app.service(Files::new("/files", base_dir).show_files_listing());
        app = app.service(cdn::services(&config));

        app
    });

    for binding in config.bindings.iter() {
        server = server
            .bind(binding.clone())
            .chain_err(|| "Error binding the actix server")?;
    }

    server
        .run()
        .await
        .chain_err(|| "Error starting the actix server")
}

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    logging::init();

    if let Err(ref e) = run().await {
        e.log();
        exit(1);
    }
}
