use rquickjs::{function::Opt, Ctx, Exception, FromJs, Object, Result, Value};

use super::{utils, DatabaseRaw};
use crate::utils::result::ResultExt;

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct Database {
    #[qjs(skip_trace)]
    location: String,
    #[qjs(skip_trace)]
    raw: Option<DatabaseRaw>,
}

impl Database {
    fn raw(&self, ctx: &Ctx<'_>) -> Result<&DatabaseRaw> {
        self.raw
            .as_ref()
            .ok_or_else(|| Exception::throw_message(ctx, "Database is not open"))
    }

    async fn open_(ctx: &Ctx<'_>, location: &str) -> Result<DatabaseRaw> {
        let location = location.to_owned();
        let raw = utils::asyncify(ctx, move || DatabaseRaw::open(&location)).await?;
        Ok(raw)
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl Database {
    #[qjs(constructor)]
    pub fn new(location: String, options: Opt<ConnectionOptions>) -> Self {
        Self {
            location,
            raw: None,
        }
    }

    async fn open(&mut self, ctx: Ctx<'_>) -> rquickjs::Result<()> {
        self.raw = Some(Self::open_(&ctx, &self.location).await?);
        Ok(())
    }

    async fn prepare(&self, sql: String) -> Result<()> {
        todo!()
    }

    async fn exec(&self, ctx: Ctx<'_>, sql: String) -> Result<()> {
        let raw = self.raw(&ctx)?.clone();
        let sql = sql.to_owned();
        utils::asyncify(&ctx, move || {
            raw.execute(&sql)?;
            Ok(())
        })
        .await?;
        Ok(())
    }

    async fn close(&mut self, ctx: Ctx<'_>) -> Result<()> {
        match self.raw.take() {
            Some(raw) => {
                utils::asyncify(&ctx, move || raw.close()).await?;
                Ok(())
            }
            None => Err(Exception::throw_message(
                &ctx,
                "Connection is already closed",
            )),
        }
    }
}

pub struct ConnectionOptions {}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {}
    }
}

impl<'js> FromJs<'js> for ConnectionOptions {
    fn from_js(_ctx: &Ctx<'js>, value: Value<'js>) -> Result<Self> {
        let obj = value.get::<Object<'js>>()?;
        Ok(Self {})
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
                        import { Database } from "sqlite";

                        export async function test() {
                            const db = new Database(":memory:");
                            await db.open();
                            await db.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT);");
                            await db.exec("INSERT INTO test (name) VALUES ('test');");
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
