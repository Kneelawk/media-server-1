use crate::error::{ErrorKind, Result};
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref FILE_EXTENSION_PATTERN: Regex = Regex::new(r#".*\.(?P<ext>[^.]+)$"#).unwrap();
}

pub fn file_extension(path: &str) -> Option<&str> {
    FILE_EXTENSION_PATTERN
        .captures(path)
        .and_then(|c| c.name("ext"))
        .map(|m| m.as_str())
}

/*
 * Copied from actix-files-0.5.0/src/error.rs to make sure responses stay the
 * same for limited files.
 */

pub fn parse_path(path: &str, hidden_files: bool) -> Result<PathBuf> {
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
