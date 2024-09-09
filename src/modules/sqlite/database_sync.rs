use rquickjs::{function::Opt, Ctx, Exception, FromJs, Object, Result, Value};

use super::DatabaseRaw;
use crate::utils::result::ResultExt;

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct DatabaseSync {
    #[qjs(skip_trace)]
    location: String,
    #[qjs(skip_trace)]
    raw: Option<DatabaseRaw>,
}

impl DatabaseSync {
    fn raw(&self, ctx: &Ctx<'_>) -> Result<&DatabaseRaw> {
        self.raw
            .as_ref()
            .ok_or_else(|| Exception::throw_message(ctx, "Database is not open"))
    }

    fn open_(ctx: &Ctx<'_>, location: &str) -> Result<DatabaseRaw> {
        DatabaseRaw::open(location).or_throw(ctx)
    }

    fn prepare_(ctx: Ctx<'_>, db: DatabaseRaw, sql: &str) -> Result<()> {
        Ok(())
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl DatabaseSync {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'_>, location: String, options: Opt<ConnectionOptions>) -> Result<Self> {
        let options = options.0.unwrap_or_default();

        let raw = if options.open {
            Some(Self::open_(&ctx, &location)?)
        } else {
            None
        };

        Ok(Self { location, raw })
    }

    fn open(&mut self, ctx: Ctx<'_>) -> rquickjs::Result<()> {
        self.raw = Some(Self::open_(&ctx, &self.location)?);
        Ok(())
    }

    fn prepare(&self, sql: String) -> Result<()> {
        todo!()
    }

    fn exec(&self, ctx: Ctx<'_>, sql: String) -> Result<()> {
        let raw = self.raw(&ctx)?;
        raw.execute(&sql).or_throw(&ctx)
    }

    fn close(&mut self, ctx: Ctx<'_>) -> Result<()> {
        match self.raw.take() {
            Some(raw) => raw.close().or_throw(&ctx),
            None => Err(Exception::throw_message(
                &ctx,
                "Connection is already closed",
            )),
        }
    }
}

pub struct ConnectionOptions {
    open: bool,
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self { open: true }
    }
}

impl<'js> FromJs<'js> for ConnectionOptions {
    fn from_js(_ctx: &Ctx<'js>, value: Value<'js>) -> Result<Self> {
        let obj = value.get::<Object<'js>>()?;
        Ok(Self {
            open: obj.get("open").unwrap_or(true),
        })
    }
}

#[cfg(test)]
mod tests {
    use rquickjs::CatchResultExt;

    use crate::sqlite::SqliteModule;
    use crate::test::{call_test, test_async_with, ModuleEvaluator};

    #[tokio::test]
    async fn test_open_exec() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<SqliteModule>(ctx.clone(), "sqlite")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { DatabaseSync } from "sqlite";

                        export function test() {
                            const db = new DatabaseSync(":memory:");
                            db.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT);");
                            db.exec("INSERT INTO test (name) VALUES ('test');");
                            return "ok";
                        }
                    "#,
                )
                .await
                .catch(&ctx)
                .unwrap();

                let result = call_test::<String, _>(&ctx, &module, ()).await;
                assert_eq!(result, "ok");
            })
        })
        .await;
    }
}
