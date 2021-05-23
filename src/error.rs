use crate::util::w_err;
use actix_web::{
    dev::HttpResponseBuilder, http::StatusCode, HttpResponse,
    ResponseError,
};
use error_chain::ChainedError;
use std::borrow::Cow;

error_chain! {
    errors {
        ConfigLoadError(msg: Cow<'static, str>) {
            display("Error loading config: {}", msg)
        }
        FilesLimiterError {}
        UriSegmentError {}
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            ErrorKind::FilesLimiterError => StatusCode::NOT_FOUND,
            ErrorKind::UriSegmentError => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        if let Some(json) = self.handle() {
            HttpResponseBuilder::new(self.status_code()).json(&w_err(json))
        } else {
            HttpResponse::new(self.status_code())
        }
    }
}

impl Error {
    fn handle(&self) -> Option<JsonError> {
        match self.0 {
            ErrorKind::FilesLimiterError => None,
            ErrorKind::UriSegmentError => None,
            _ => {
                self.log();
                Some(JsonError::InternalServerError)
            }
        }
    }

    pub fn log(&self) {
        error!("{}", self.display_chain().to_string());
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum JsonError {
    InternalServerError,
}
