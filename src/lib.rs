#[cfg(feature = "console")]
pub use rquickjs_extra_console as console;

#[cfg(feature = "os")]
pub use rquickjs_extra_os as os;

#[cfg(feature = "sqlite")]
pub use rquickjs_extra_sqlite as sqlite;

#[cfg(feature = "timers")]
pub use rquickjs_extra_timers as timers;

#[cfg(feature = "url")]
pub use rquickjs_extra_url as url;
