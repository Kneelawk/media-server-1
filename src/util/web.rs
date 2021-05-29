use crate::util::{w_err, w_ok};
use actix_web::{dev::HttpResponseBuilder, http::StatusCode, HttpResponse};

/// Constructs a JSON Err response with the specified status code.
pub fn json_err<E: serde::Serialize>(status: StatusCode, json: E) -> HttpResponse {
    HttpResponseBuilder::new(status).json(w_err(json))
}

/// Constructs a JSON Ok response
pub fn json_ok<T: serde::Serialize>(json: T) -> HttpResponse {
    HttpResponse::Ok().json(w_ok(json))
}

/// Constructs a JSON Ok response but with the specified status code. This us
/// usually used for specific errors that are not errors in the same way JSON
/// Err responses are. This includes things like a file index describing a file
/// that does not exist with a 404 response code but an otherwise Ok response.
pub fn json_ok_status<T: serde::Serialize>(status: StatusCode, json: T) -> HttpResponse {
    HttpResponseBuilder::new(status).json(w_ok(json))
}
