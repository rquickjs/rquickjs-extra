use rquickjs::{
    Class, Ctx, Result,
    function::{Async, Func},
    module::{Declarations, Exports, ModuleDef},
};
use rquickjs_extra_utils::module::export_default;

pub use self::argument::Argument;
pub use self::database::Database;
pub use self::open::{OpenOptions, open};
pub use self::statement::Statement;
pub use self::value::Value;

mod argument;
mod database;
mod open;
mod statement;
mod value;

pub struct SqliteModule;

impl ModuleDef for SqliteModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare(stringify!(Database))?;
        declare.declare("open")?;
        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<Database>::define(default)?;

            default.set("open", Func::from(Async(open::open)))?;

            Ok(())
        })?;
        Ok(())
    }
}
