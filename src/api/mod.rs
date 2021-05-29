mod index;
mod status;

use crate::config::Config;
use actix_web::{web, Scope};

pub fn service(config: &Config) -> Scope {
    web::scope("api/v1")
        .service(index::service(config))
        .service(status::get_status)
}
