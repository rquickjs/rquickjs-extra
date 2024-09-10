use rquickjs::{
    function::{Async, Func},
    module::{Declarations, Exports, ModuleDef},
    Class, Ctx, Result,
};

pub use self::argument::Argument;
pub use self::database::Database;
pub use self::open::open;
pub use self::statement::Statement;
pub use self::value::Value;
use crate::utils::module::export_default;

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
