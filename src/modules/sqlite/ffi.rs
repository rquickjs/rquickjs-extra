pub use libsqlite3_sys::*;

// Not exposed by libsqlite3_sys since they consider they don't
// need it, but we do are in a GC context and we can't easily
// predict if a statement will still be alive when the connection
// is closed or recycled.
extern "C" {
    pub fn sqlite3_close_v2(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
