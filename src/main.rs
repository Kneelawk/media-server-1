#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod config;
mod error;
mod logging;
mod util;

use crate::{
    config::Config,
    error::{Result, ResultExt},
};
use actix_files::Files;
use actix_web::{App, HttpServer};
use std::process::exit;

async fn run() -> Result<()> {
    let config = Config::load()?;

    util::ffmpeg::init_ffmpeg()?;

    let base_dir = config.base_dir.clone();

    let mut server = HttpServer::new(move || {
        let base_dir = base_dir.clone();

        App::new().service(Files::new("/files", base_dir).show_files_listing())
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
