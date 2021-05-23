use crate::{
    config::Config,
    error::{Error, ErrorKind, Result},
};
use actix_files::Files;
use actix_service::ServiceFactory;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Scope,
};
use futures::{
    future,
    future::{ok, Either, Ready},
    task::{Context, Poll},
};
use std::{future::Future, path::PathBuf, pin::Pin, result};

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
    web::scope("/files")
        .wrap(FilesLimiter {
            config: config.clone(),
        })
        .service(Files::new("", &config.base_dir).show_files_listing())
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

        if self.config.is_legal_path(&path_str) {
            Either::Right(Box::pin(self.service.call(req)))
        } else {
            Either::Left(ok(
                req.error_response(Error::from_kind(ErrorKind::FilesLimiterError))
            ))
        }
    }
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
