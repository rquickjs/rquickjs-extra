use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Class, Ctx, Result,
};

use self::argument::Argument;
use self::database::Database;
use crate::utils::module::export_default;

mod argument;
mod database;
mod error;
mod open;

pub struct SqliteModule;

impl ModuleDef for SqliteModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare(stringify!(Database))?;
        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<Database>::define(default)?;

            Ok(())
        })?;
        Ok(())
    }
}
