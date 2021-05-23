use crate::{
    config::Config,
    error::{Error, ErrorKind, Result},
};
use actix_files::{Directory, Files};
use actix_service::ServiceFactory;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, HttpRequest, HttpResponse, Scope,
};
use futures::{
    future,
    future::{ok, Either, Ready},
    task::{Context, Poll},
};
use path_slash::PathExt;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::{
    fmt::Write,
    future::Future,
    io,
    path::{Path, PathBuf},
    pin::Pin,
    result,
};

lazy_static! {
    static ref PATH_SET: AsciiSet = NON_ALPHANUMERIC
        .remove(b'/')
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'+');
}

pub fn service(
    config: &Config,
) -> Scope<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let config_copy = config.clone();

    web::scope("/files")
        .wrap(FilesLimiter {
            config: config.clone(),
        })
        .service(
            Files::new("", &config.base_dir)
                .show_files_listing()
                .files_listing_renderer(move |dir, req| directory_listing(&config_copy, dir, req)),
        )
}

struct FilesLimiter {
    config: Config,
}

impl<S, B> Transform<S> for FilesLimiter
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = FilesLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<result::Result<Self::Transform, ()>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(FilesLimiterMiddleware {
            service,
            config: self.config.clone(),
        })
    }
}

struct FilesLimiterMiddleware<S> {
    service: S,
    config: Config,
}

impl<S, B> Service for FilesLimiterMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Either<
        Ready<result::Result<Self::Response, Self::Error>>,
        Pin<Box<dyn Future<Output = result::Result<Self::Response, Self::Error>>>>,
    >;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<result::Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let real_path = match parse_path(req.match_info().path(), false) {
            Ok(item) => item,
            Err(e) => return Either::Left(ok(req.error_response(e))),
        };

        let path_str = real_path.to_string_lossy();

        trace!("Limiter: Path: {}", &path_str);

        if self.config.is_legal_path(&path_str) {
            trace!("Limiter: Accepted");
            Either::Right(Box::pin(self.service.call(req)))
        } else {
            trace!("Limiter: Rejected");
            Either::Left(ok(
                req.error_response(Error::from_kind(ErrorKind::FilesLimiterError))
            ))
        }
    }
}

/*
 * Copied from actix-files-0.5.0/src/directory.rs
 */

fn directory_listing(
    config: &Config,
    dir: &Directory,
    req: &HttpRequest,
) -> io::Result<ServiceResponse> {
    let path = percent_decode_str(req.path())
        .decode_utf8_lossy()
        .into_owned();
    let index_of = format!("Index of {}", &path);
    let mut body = String::new();
    let base = Path::new(&path);

    for entry in dir.path.read_dir()? {
        if dir.is_visible(&entry) {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let stripped = match entry_path.strip_prefix(&dir.path) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let path_str = stripped.to_string_lossy();
            trace!("Listing: Path: {}", &path_str);

            if config.is_legal_path(&path_str) {
                trace!("Listing: Accepted");
                let href = base.join(&stripped).to_slash_lossy();

                // if file is a directory, add '/' to the end of the name
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let _ = write!(
                            body,
                            "<li><a href=\"{}\">{}/</a></li>",
                            utf8_percent_encode(&href, &PATH_SET),
                            v_htmlescape::escape(&entry.file_name().to_string_lossy()),
                        );
                    } else {
                        let _ = write!(
                            body,
                            "<li><a href=\"{}\">{}</a></li>",
                            utf8_percent_encode(&href, &PATH_SET),
                            v_htmlescape::escape(&entry.file_name().to_string_lossy()),
                        );
                    }
                } else {
                    continue;
                }
            } else {
                trace!("Listing: Rejected");
                continue;
            }
        }
    }

    let html = format!(
        "<html>\
         <head><title>{}</title></head>\
         <body><h1>{}</h1>\
         <ul>\
         {}\
         </ul></body>\n</html>",
        index_of, index_of, body
    );
    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
    ))
}

/*
 * Copied from actix-files-0.5.0/src/error.rs to make sure responses stay the
 * same for limited files.
 */

fn parse_path(path: &str, hidden_files: bool) -> Result<PathBuf> {
    let mut buf = PathBuf::new();

    for segment in path.split('/') {
        if segment == ".." {
            buf.pop();
        } else if !hidden_files && segment.starts_with('.') {
            bail!(ErrorKind::UriSegmentError)
        } else if segment.starts_with('*') {
            bail!(ErrorKind::UriSegmentError)
        } else if segment.ends_with(':') {
            bail!(ErrorKind::UriSegmentError)
        } else if segment.ends_with('>') {
            bail!(ErrorKind::UriSegmentError)
        } else if segment.ends_with('<') {
            bail!(ErrorKind::UriSegmentError)
        } else if segment.is_empty() {
            continue;
        } else if cfg!(windows) && segment.contains('\\') {
            bail!(ErrorKind::UriSegmentError)
        } else {
            buf.push(segment)
        }
    }

    Ok(buf)
}
