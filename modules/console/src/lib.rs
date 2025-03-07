use std::fmt::Write;

use rquickjs::{class::Trace, function::Rest, Ctx, Error, JsLifetime, Result, Value};

pub use self::formatter::Formatter;

mod formatter;

const TARGET: &str = "console";

/// A console object to print messages to the [`log`] crate.
///
/// # Example
/// ```rust
/// use rquickjs::{Context, Runtime};
/// use rquickjs_extra_console::{Console, Formatter};
///
/// fn main() {
///     let rt = Runtime::new().unwrap();
///     let ctx = Context::full(&rt).unwrap();
///
///     ctx.with(|ctx| {
///         let console = Console::new("hello", Formatter::default());
///         ctx.globals().set("console", console).unwrap();
///         ctx.eval::<(), _>("console.log('test')").unwrap();
///      })
/// }
/// ```
///
/// [`log`]: https://docs.rs/log
#[derive(Clone, Trace, JsLifetime)]
#[rquickjs::class(frozen)]
pub struct Console {
    target: String,
    formatter: Formatter,
}

impl Console {
    pub fn new(target: &str, formatter: Formatter) -> Self {
        Self {
            target: target.to_string(),
            formatter,
        }
    }

    fn print(&self, level: log::Level, values: Rest<Value<'_>>) -> Result<()> {
        let mut message = String::new();
        for (i, value) in values.0.into_iter().enumerate() {
            if i > 0 {
                write!(&mut message, ", ").map_err(|_| Error::Unknown)?
            }
            self.formatter.format(&mut message, value)?
        }
        log::log!(target: &self.target, level, "{}", message);
        Ok(())
    }
}

#[rquickjs::methods]
impl Console {
    fn debug(&self, values: Rest<Value<'_>>) -> Result<()> {
        self.print(log::Level::Debug, values)
    }

    fn log(&self, values: Rest<Value<'_>>) -> Result<()> {
        self.print(log::Level::Info, values)
    }

    fn warn(&self, values: Rest<Value<'_>>) -> Result<()> {
        self.print(log::Level::Warn, values)
    }

    fn error(&self, values: Rest<Value<'_>>) -> Result<()> {
        self.print(log::Level::Error, values)
    }
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    globals.set(
        "console",
        Console::new(TARGET, Formatter::builder().max_depth(3).build()),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rquickjs_extra_test::test_with;

    use super::*;

    #[test]
    fn test_console() {
        test_with(|ctx| {
            let console = Console::new("hello", Formatter::default());
            ctx.globals().set("console", console).unwrap();

            let result = ctx.eval::<(), _>("console.log('test')");
            assert!(result.is_ok());
        })
    }
}
