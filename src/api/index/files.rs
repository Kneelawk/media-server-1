use crate::{
    config::Config,
    error::{
        Error, ErrorKind,
        ErrorKind::{FilesIndexUnknownError, InvalidMethodError},
        Result, ResultExt,
    },
    util::{
        path::parse_path,
        web::{json_ok, json_ok_status},
    },
};
use actix_service::ServiceFactory;
use actix_web::{
    dev::{AppService, HttpServiceFactory, ResourceDef, Service, ServiceRequest, ServiceResponse},
    error::Error as WebError,
    http::{header, Method, StatusCode},
    web, HttpRequest, HttpResponse, Scope,
};
use core::result;
use futures::future::{ok, Ready};
use path_slash::PathExt;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::{
    io,
    path::Path,
    task::{Context, Poll},
};

const CDN_FILES_URL: &'static str = "/cdn/files";

lazy_static! {
    static ref PATH_SET: AsciiSet = NON_ALPHANUMERIC
        .remove(b'/')
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'+');
}

pub fn files(config: &Config) -> Scope {
    web::scope("files").service(FilesIndex {
        config: config.clone(),
    })
}

struct FilesIndex {
    config: Config,
}

struct FilesIndexService {
    config: Config,
}

impl HttpServiceFactory for FilesIndex {
    fn register(self, config: &mut AppService) {
        let rdef = if config.is_root() {
            ResourceDef::root_prefix("")
        } else {
            ResourceDef::prefix("")
        };

        config.register_service(rdef, None, self, None);
    }
}

impl ServiceFactory for FilesIndex {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = WebError;
    type Config = ();
    type Service = FilesIndexService;
    type InitError = ();
    type Future = Ready<result::Result<FilesIndexService, Self::InitError>>;

    fn new_service(&self, _cfg: Self::Config) -> Self::Future {
        ok(FilesIndexService {
            config: self.config.clone(),
        })
    }
}

impl Service for FilesIndexService {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = WebError;
    type Future = Ready<result::Result<ServiceResponse, WebError>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let is_method_valid = matches!(*req.method(), Method::HEAD | Method::GET);

        if !is_method_valid {
            return ok(req.error_response(Error::from_kind(InvalidMethodError)));
        }

        let full_path_str = req.path().to_string();

        let relative_path_str = req.match_info().path().to_string();
        let url_encoded_relative_path =
            utf8_percent_encode(&relative_path_str, &PATH_SET).to_string();

        let (http, _) = req.into_parts();

        let relative_path = match parse_path(&relative_path_str, false) {
            Ok(p) => p,
            Err(e) => return ok(error_response(e, &http)),
        };

        let file_path = match self.config.base_dir.join(&relative_path).canonicalize() {
            Ok(p) => p,
            Err(e) => {
                return ok(response_from_io_error(
                    e,
                    &http,
                    relative_path_str,
                    url_encoded_relative_path,
                    relative_path
                        .file_name()
                        .map_or("".to_string(), |s| s.to_string_lossy().to_string()),
                ));
            }
        };

        if self.config.is_legal_path(&relative_path.to_string_lossy()) {
            if file_path.is_dir() {
                // we want to redirect to directories
                if !full_path_str.ends_with('/') {
                    let redirect_to = format!("{}/", full_path_str);

                    return ok(ServiceResponse::new(
                        http,
                        HttpResponse::Found()
                            .header(header::LOCATION, redirect_to)
                            .body("")
                            .into_body(),
                    ));
                }

                match render_directory(
                    &self.config,
                    &http,
                    &file_path,
                    &relative_path,
                    relative_path_str,
                    url_encoded_relative_path,
                    relative_path
                        .file_name()
                        .map_or("".to_string(), |s| s.to_string_lossy().to_string()),
                ) {
                    Ok(res) => ok(res),
                    Err(e) => return ok(error_response(e, &http)),
                }
            } else {
                let json = JsonEntryInfo {
                    detail: JsonEntryDetail::File {
                        url: format!("{}{}", CDN_FILES_URL, url_encoded_relative_path),
                    },
                    name: relative_path
                        .file_name()
                        .map_or("".to_string(), |s| s.to_string_lossy().to_string()),
                    path: url_encoded_relative_path,
                    path_pretty: relative_path_str,
                };

                ok(ServiceResponse::new(http, json_ok(json)))
            }
        } else {
            ok(ServiceResponse::new(
                http,
                json_ok_status(
                    StatusCode::NOT_FOUND,
                    JsonEntryInfo {
                        detail: JsonEntryDetail::Error {
                            error: JsonIndexError::NotFound,
                        },
                        name: relative_path
                            .file_name()
                            .map_or("".to_string(), |s| s.to_string_lossy().to_string()),
                        path: url_encoded_relative_path,
                        path_pretty: relative_path_str,
                    },
                ),
            ))
        }
    }
}

fn render_directory(
    config: &Config,
    http: &HttpRequest,
    file_path: &Path,
    relative_path: &Path,
    relative_path_str: String,
    url_encoded_relative_path: String,
    file_name: String,
) -> Result<ServiceResponse> {
    let url_base = Path::new(http.path());
    let path_base = Path::new("/").join(relative_path);
    let mut children_vec = vec![];

    let read_dir = match file_path.read_dir() {
        Ok(ok) => ok,
        Err(e) => {
            return Ok(response_from_io_error(
                e,
                http,
                relative_path_str,
                url_encoded_relative_path,
                file_name,
            ));
        }
    };

    for entry in read_dir {
        let entry = entry.chain_err(|| unknown_err(http, "Unwrapping DirEntry"))?;
        let entry_path = entry.path();

        let stripped_path = match entry_path.strip_prefix(file_path) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let stripped_path_str = stripped_path.to_string_lossy();

        if config.is_legal_path(&stripped_path_str) {
            let url_path = url_base.join(stripped_path);
            let path_path = path_base.join(stripped_path);

            if let Ok(metadata) = entry.metadata() {
                let name = entry.file_name().to_string_lossy().to_string();
                let url = utf8_percent_encode(&url_path.to_slash_lossy(), &PATH_SET).to_string();
                let path = utf8_percent_encode(&path_path.to_slash_lossy(), &PATH_SET).to_string();

                if metadata.is_dir() {
                    children_vec.push(JsonDirectoryChild {
                        name,
                        ty: JsonEntryType::Directory,
                        url: format!("{}/", url),
                        path: format!("{}/", path),
                    })
                } else {
                    children_vec.push(JsonDirectoryChild {
                        name,
                        ty: JsonEntryType::File,
                        url,
                        path,
                    })
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    let json = JsonEntryInfo {
        detail: JsonEntryDetail::Directory {
            children: children_vec,
        },
        name: file_name,
        path: url_encoded_relative_path,
        path_pretty: relative_path_str,
    };

    Ok(ServiceResponse::new(http.clone(), json_ok(json)))
}

fn unknown_err(req: &HttpRequest, msg: &str) -> ErrorKind {
    FilesIndexUnknownError(format!("{}: {}", msg, req.path()).into())
}

fn response_from_io_error(
    e: io::Error,
    http: &HttpRequest,
    relative_path: String,
    url_encoded_relative_path: String,
    file_name: String,
) -> ServiceResponse {
    if matches!(
        e.kind(),
        io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied
    ) {
        ServiceResponse::new(
            http.clone(),
            json_ok_status(
                match e.kind() {
                    io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                    _ => unreachable!(),
                },
                JsonEntryInfo {
                    detail: JsonEntryDetail::Error {
                        error: match e.kind() {
                            io::ErrorKind::NotFound => JsonIndexError::NotFound,
                            io::ErrorKind::PermissionDenied => JsonIndexError::Forbidden,
                            _ => unreachable!(),
                        },
                    },
                    name: file_name,
                    path: url_encoded_relative_path,
                    path_pretty: relative_path,
                },
            ),
        )
    } else {
        error_response(unknown_err(http, "Canonicalize path").into(), http)
    }
}

fn error_response(e: Error, http: &HttpRequest) -> ServiceResponse {
    ServiceResponse::from_err(e, http.clone())
}

#[derive(Debug, Serialize)]
struct JsonEntryInfo {
    detail: JsonEntryDetail,
    name: String,
    path: String,
    path_pretty: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum JsonEntryDetail {
    Directory { children: Vec<JsonDirectoryChild> },
    Error { error: JsonIndexError },
    File { url: String },
}

#[derive(Debug, Serialize)]
struct JsonDirectoryChild {
    name: String,
    #[serde(rename = "type")]
    ty: JsonEntryType,
    url: String,
    path: String,
}

#[derive(Debug, Serialize)]
enum JsonEntryType {
    Directory,
    File,
}

#[derive(Debug, Serialize)]
enum JsonIndexError {
    NotFound,
    Forbidden,
}
