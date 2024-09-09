use std::ffi::{c_char, c_int, CStr, CString};

use crate::utils::result::ResultExt;

use super::{error, ffi};

#[inline]
pub unsafe fn c_str_to_string(str: *const c_char) -> String {
    CStr::from_ptr(str).to_string_lossy().into_owned()
}

#[inline]
pub fn str_to_cstring(str: &str) -> CString {
    CString::new(str).expect("Failed to convert string to CString")
}

#[inline]
pub fn str_to_c_char(str: &str) -> error::Result<(*const c_char, c_int)> {
    let len = len_as_c_int(str.len())?;
    let ptr = str.as_ptr().cast::<c_char>();
    Ok((ptr, len))
}

pub fn len_as_c_int(len: usize) -> error::Result<c_int> {
    if len >= (c_int::MAX as usize) {
        Err(error::Error {
            inner: ffi::Error::new(ffi::SQLITE_TOOBIG),
            message: None,
        })
    } else {
        Ok(len as c_int)
    }
}

#[inline]
pub async fn asyncify<'js, F, R>(ctx: &rquickjs::Ctx<'js>, f: F) -> rquickjs::Result<R>
where
    F: FnOnce() -> error::Result<R> + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .or_throw_msg(ctx, "Cannot send task to worker pool")?
        .or_throw(ctx)
}
