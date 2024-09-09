use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Class, Ctx, Result,
};

use self::argument::Argument;
use self::database::Database;
use self::database_raw::DatabaseRaw;
use self::database_sync::DatabaseSync;
use self::statement_raw::StatementRaw;
use crate::utils::module::export_default;

mod argument;
mod database;
mod database_raw;
mod database_sync;
mod error;
mod ffi;
mod statement;
mod statement_raw;
mod statement_sync;
mod utils;

pub struct SqliteModule;

impl ModuleDef for SqliteModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare(stringify!(Database))?;
        declare.declare(stringify!(DatabaseSync))?;
        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<Database>::define(default)?;
            Class::<DatabaseSync>::define(default)?;

            Ok(())
        })?;
        Ok(())
    }
}
