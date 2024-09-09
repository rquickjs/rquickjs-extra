use std::{error, ffi::c_int, fmt};

use super::{ffi, utils};

#[derive(Debug)]
pub struct Error {
    pub inner: ffi::Error,
    pub message: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(formatter)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.inner)
    }
}

#[cold]
pub fn error_from_code(code: c_int, message: Option<String>) -> Error {
    Error {
        inner: ffi::Error::new(code),
        message,
    }
}

#[cold]
pub unsafe fn error_from_handle(db: *mut ffi::sqlite3, code: c_int) -> Error {
    let message = if db.is_null() {
        None
    } else {
        Some(utils::c_str_to_string(ffi::sqlite3_errmsg(db)))
    };
    Error {
        inner: ffi::Error::new(code),
        message,
    }
}

pub type Result<T> = std::result::Result<T, Error>;
