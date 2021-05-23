mod files;

use crate::config::Config;
use actix_web::{web, Scope};

pub fn services(config: &Config) -> Scope {
    web::scope("/cdn").service(files::service(config))
}
