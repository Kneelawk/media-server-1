use error_chain::ChainedError;
use std::borrow::Cow;

error_chain! {
    errors {
        ConfigLoadError(msg: Cow<'static, str>) {
            display("Error loading config: {}", msg)
        }
    }
}

impl Error {
    pub fn log(&self) {
        error!("{}", self.display_chain().to_string());
    }
}
