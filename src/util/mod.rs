#[cfg(feature = "ffmpeg")]
pub mod ffmpeg;
pub mod path;
pub mod web;

// Result wrapper functions

pub fn w_err<T>(t: T) -> Result<(), T> {
    Err(t)
}

pub fn w_ok<T>(t: T) -> Result<T, ()> {
    Ok(t)
}
