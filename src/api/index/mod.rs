use actix_web::{web, Scope};

use crate::config::Config;

mod files;

pub fn service(config: &Config) -> Scope {
    web::scope("index").service(files::files(config))
}
