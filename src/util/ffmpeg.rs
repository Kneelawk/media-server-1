use crate::{
    error::{Result, ResultExt},
    logging,
};

/// Initializes ffmpeg and the custom logging callback.
pub fn init_ffmpeg() -> Result<()> {
    ffmpeg4::init().chain_err(|| "Initializing ffmpeg")?;
    unsafe { ffmpeg4_sys::av_log_set_callback(Some(logging::log_callback)) };

    Ok(())
}
