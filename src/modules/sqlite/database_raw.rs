use std::{os::raw::c_int, ptr};

use super::{error, ffi, utils, StatementRaw};

#[derive(Clone, Copy, Debug)]
pub struct OpenFlags(c_int);

impl OpenFlags {
    #[inline]
    pub fn new() -> Self {
        OpenFlags(0)
    }

    pub fn with_create(mut self) -> Self {
        self.0 |= ffi::SQLITE_OPEN_CREATE;
        self
    }

    pub fn with_full_mutex(mut self) -> Self {
        self.0 |= ffi::SQLITE_OPEN_FULLMUTEX;
        self
    }

    pub fn with_read_write(mut self) -> Self {
        self.0 |= ffi::SQLITE_OPEN_READWRITE;
        self
    }

    pub fn with_uri(mut self) -> Self {
        self.0 |= ffi::SQLITE_OPEN_URI;
        self
    }

    pub fn with_extended_result_code(mut self) -> Self {
        self.0 |= ffi::SQLITE_OPEN_URI;
        self
    }
}

impl Default for OpenFlags {
    #[inline]
    fn default() -> Self {
        Self::new()
            .with_create()
            .with_full_mutex()
            .with_read_write()
            .with_uri()
    }
}

/// A connection to a SQLite database.
/// We make a few assumptions about the SQLite library:
/// - SQlite was compiled in serialized mode so that we can safely share the connection
///   and related objects between threads.
/// - SQlite is a recent version that supports extended result codes.
pub struct DatabaseRaw {
    db: *mut ffi::sqlite3,
    owned: bool,
}

impl DatabaseRaw {
    pub unsafe fn new(db: *mut ffi::sqlite3, owned: bool) -> Self {
        Self { db, owned }
    }

    pub fn open(path: &str) -> error::Result<Self> {
        Self::open_with_flags(path, OpenFlags::default())
    }

    pub fn open_with_flags(path: &str, mut flags: OpenFlags) -> error::Result<Self> {
        let c_path = utils::str_to_cstring(path);

        flags = flags.with_extended_result_code();

        unsafe {
            let mut db: *mut ffi::sqlite3 = ptr::null_mut();
            let r = ffi::sqlite3_open_v2(c_path.as_ptr(), &mut db, flags.0, std::ptr::null());
            if r != ffi::SQLITE_OK {
                let e = if db.is_null() {
                    error::error_from_code(r, Some(c_path.to_string_lossy().to_string()))
                } else {
                    let mut e = error::error_from_handle(db, r);
                    if e.inner.code == ffi::ErrorCode::CannotOpen {
                        e.message = Some(format!(
                            "{}: {}",
                            e.message.unwrap_or_else(|| "Cannot open".to_string()),
                            c_path.to_string_lossy()
                        ));
                    }
                    ffi::sqlite3_close(db);
                    e
                };

                return Err(e);
            }

            // Attempt to turn on extended results code. Don't fail if we can't.
            ffi::sqlite3_extended_result_codes(db, 1);

            // Set busy timeout to 5 seconds
            let r = ffi::sqlite3_busy_timeout(db, 5000);
            if r != ffi::SQLITE_OK {
                let e = error::error_from_handle(db, r);
                ffi::sqlite3_close(db);
                return Err(e);
            }

            Ok(Self::new(db, true))
        }
    }

    pub fn decode_result(&self, code: c_int) -> error::Result<()> {
        unsafe { Self::decode_result_raw(self.db, code) }
    }

    #[inline]
    unsafe fn decode_result_raw(db: *mut ffi::sqlite3, code: c_int) -> error::Result<()> {
        if code == ffi::SQLITE_OK {
            Ok(())
        } else {
            Err(error::error_from_handle(db, code))
        }
    }

    pub fn close(mut self) -> error::Result<()> {
        self.close_()
    }

    fn close_(&mut self) -> error::Result<()> {
        if self.db.is_null() {
            return Ok(());
        }
        if !self.owned {
            self.db = ptr::null_mut();
            return Ok(());
        }
        unsafe {
            let r = ffi::sqlite3_close_v2(self.db);
            let r = Self::decode_result_raw(self.db, r);
            if r.is_ok() {
                self.db = ptr::null_mut();
            }
            r
        }
    }

    pub fn execute(&self, sql: &str) -> error::Result<()> {
        let c_sql = utils::str_to_cstring(sql);
        unsafe {
            let r = ffi::sqlite3_exec(
                self.db,
                c_sql.as_ptr(),
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            Self::decode_result_raw(self.db, r)
        }
    }

    pub fn prepare(&mut self, sql: &str) -> error::Result<StatementRaw> {
        let mut c_stmt: *mut ffi::sqlite3_stmt = ptr::null_mut();
        let c_sql = utils::str_to_cstring(sql);
        unsafe {
            let r =
                ffi::sqlite3_prepare_v2(self.db, c_sql.as_ptr(), -1, &mut c_stmt, ptr::null_mut());
            Self::decode_result_raw(self.db, r)?;
            Ok(StatementRaw::new(c_stmt, self.clone()))
        }
    }
}

unsafe impl Send for DatabaseRaw {}

impl Clone for DatabaseRaw {
    fn clone(&self) -> Self {
        Self {
            db: self.db,
            owned: false,
        }
    }
}

impl Drop for DatabaseRaw {
    fn drop(&mut self) {
        if let Err(err) = self.close_() {
            tracing::error!("Error closing SQLite connection: {}", err);
        }
    }
}
