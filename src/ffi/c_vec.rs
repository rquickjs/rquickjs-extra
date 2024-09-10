use rquickjs::{Result, TypedArray, Value};

use crate::utils::result::ResultExt;

#[derive(Debug)]
pub struct CVec<'js> {
    ptr: *const u8,
    len: usize,
    #[allow(dead_code)]
    value: Value<'js>,
}

#[allow(dead_code)]
impl<'js> CVec<'js> {
    pub fn from_array(array: TypedArray<'js, u8>) -> Result<Self> {
        let raw = array.as_raw().or_throw(array.ctx())?;
        Ok(Self {
            ptr: raw.ptr.as_ptr(),
            len: raw.len,
            value: array.into_value(),
        })
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}
