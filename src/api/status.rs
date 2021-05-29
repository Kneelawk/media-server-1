use crate::{config::Config, util::web::json_ok};
use actix_web::{web, HttpResponse};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

#[get("/status")]
pub async fn get_status(config: web::Data<Config>) -> HttpResponse {
    json_ok(ServerStatus {
        name: NAME,
        version: VERSION,
    })
}

#[derive(Debug, Serialize)]
struct ServerStatus {
    name: &'static str,
    version: &'static str,
}
