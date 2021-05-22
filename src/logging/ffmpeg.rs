use ffmpeg4_sys::{AVClass, __va_list_tag};
use std::{
    ffi::CStr,
    mem::transmute,
    ops::Shr,
    os::raw::{c_char, c_int, c_void},
};

pub extern "C" fn log_callback(
    ptr: *mut c_void,
    level: c_int,
    fmt: *const c_char,
    vl: *mut __va_list_tag,
) {
    // NOTE: if something is segfaulting, this place has plenty of unsafe use for
    // decoding ffmpeg log messages.

    let avc = if ptr.is_null() {
        None
    } else {
        // This transmutes ptr into a reference to a reference to an AVClass.
        unsafe { (*transmute::<_, *const *const AVClass>(ptr)).as_ref() }
    };

    let item_name = avc.and_then(|avc| avc.item_name).map(|item_name_fn| {
        // This calls a c-function pointer, attempting to get the name of the class
        // sending the log message. This then converts the resulting string pointer into
        // a Cow<str>.
        unsafe { CStr::from_ptr(item_name_fn(ptr)) }.to_string_lossy()
    });

    // This formats the ffmpeg log message the way it expects to be formatted.
    let res = match unsafe { vsprintf::vsprintf(fmt, vl) } {
        Ok(s) => s,
        Err(err) => {
            warn!("Error formatting ffmpeg log: {:?}", err);
            return;
        }
    };

    let level = level.shr(3i32).clamp(0, 7);
    let (level, level_str) = match level {
        0 => {
            // panic level
            (log::Level::Error, "PANIC")
        }
        1 => {
            // fatal level
            (log::Level::Error, "FATAL")
        }
        2 => {
            // error level
            (log::Level::Error, "ERROR")
        }
        3 => {
            // warning level
            (log::Level::Warn, "WARN")
        }
        4 => {
            // info level
            (log::Level::Info, "INFO")
        }
        5 => {
            // verbose level
            (log::Level::Debug, "VERB")
        }
        6 => {
            // debug level
            (log::Level::Debug, "DEBUG")
        }
        7 => {
            // trace level
            (log::Level::Trace, "TRACE")
        }
        _ => unreachable!("Bad log level from ffmpeg"),
    };

    if let Some(item_name) = item_name {
        log!(
            level,
            "[ffmpeg:{:>5}:{}] {}",
            level_str,
            &item_name,
            res.trim()
        );
    } else {
        log!(level, "[ffmpeg:{:>5}] {}", level_str, res.trim());
    }
}
