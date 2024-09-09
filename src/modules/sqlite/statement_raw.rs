use std::{
    ffi::{c_int, c_void},
    ptr,
};

use super::{error, ffi, utils, DatabaseRaw, Value};

pub struct StatementRaw {
    stmt: *mut ffi::sqlite3_stmt,
    db: DatabaseRaw,
}

impl StatementRaw {
    pub fn new(stmt: *mut ffi::sqlite3_stmt, db: DatabaseRaw) -> Self {
        Self { stmt, db }
    }

    #[inline]
    pub fn step(&self) -> error::Result<()> {
        let r = unsafe { ffi::sqlite3_step(self.stmt) };
        self.db.decode_result(r)
    }

    #[inline]
    pub fn reset(&mut self) -> error::Result<()> {
        let r = unsafe { ffi::sqlite3_reset(self.stmt) };
        self.db.decode_result(r)
    }

    #[inline]
    pub fn finalize(mut self) -> error::Result<()> {
        self.finalize_()
    }

    #[inline]
    fn finalize_(&mut self) -> error::Result<()> {
        let r = unsafe { ffi::sqlite3_finalize(self.stmt) };
        self.stmt = ptr::null_mut();
        self.db.decode_result(r)
    }

    pub fn bind(&self, index: usize, value: Value) -> error::Result<()> {
        let r = match value {
            Value::Null => unsafe { ffi::sqlite3_bind_null(self.stmt, index as c_int) },
            Value::Integer(i) => unsafe { ffi::sqlite3_bind_int64(self.stmt, index as c_int, i) },
            Value::Real(r) => unsafe { ffi::sqlite3_bind_double(self.stmt, index as c_int, r) },
            Value::Text(s) => unsafe {
                let (c_str, len) = utils::str_to_c_char(&s)?;
                ffi::sqlite3_bind_text(
                    self.stmt,
                    index as c_int,
                    c_str,
                    len,
                    ffi::SQLITE_TRANSIENT(),
                )
            },
            Value::Blob(b) => unsafe {
                let length = utils::len_as_c_int(b.len())?;
                ffi::sqlite3_bind_blob(
                    self.stmt,
                    index as c_int,
                    b.as_ptr().cast::<c_void>(),
                    length,
                    ffi::SQLITE_TRANSIENT(),
                )
            },
        };
        self.db.decode_result(r)
    }
}

impl Drop for StatementRaw {
    fn drop(&mut self) {
        let _ = self.finalize_();
    }
}
