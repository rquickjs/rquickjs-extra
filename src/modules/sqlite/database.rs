use rquickjs::{function::Opt, Ctx, Exception, FromJs, Object, Result, Value};
use sqlx::SqlitePool;

use crate::utils::result::ResultExt;

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct Database {
    #[qjs(skip_trace)]
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl Database {
    async fn execute(&self, ctx: Ctx<'_>, sql: String) -> Result<()> {
        todo!()
    }

    async fn close(&mut self, ctx: Ctx<'_>) -> Result<()> {
        todo!()
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
