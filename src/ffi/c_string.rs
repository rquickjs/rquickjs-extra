use std::ffi::c_char;
use std::mem;

use rquickjs::{qjs, Error, Result, String, Value};

#[derive(Debug)]
pub struct CString<'js> {
    ptr: *const c_char,
    len: usize,
    value: Value<'js>,
}

impl<'js> CString<'js> {
    pub fn from_string(string: String<'js>) -> Result<Self> {
        let mut len = mem::MaybeUninit::uninit();
        let ptr = unsafe {
            qjs::JS_ToCStringLen(
                string.ctx().as_raw().as_ptr(),
                len.as_mut_ptr(),
                string.as_raw(),
            )
        };
        if ptr.is_null() {
            // Might not ever happen but I am not 100% sure
            // so just incase check it.
            return Err(Error::Unknown);
        }
        let len = unsafe { len.assume_init() };
        Ok(Self {
            ptr,
            len,
            value: string.into_value(),
        })
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'js> Drop for CString<'js> {
    fn drop(&mut self) {
        unsafe { qjs::JS_FreeCString(self.value.ctx().as_raw().as_ptr(), self.ptr) };
    }
}
