use rquickjs::{Ctx, Result};
use rquickjs_extra_utils::result::ResultExt;
use sqlx::{Executor, SqlitePool};

use super::Statement;

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
    async fn exec(&self, ctx: Ctx<'_>, sql: String) -> Result<()> {
        sqlx::raw_sql(&sql)
            .execute(&self.pool)
            .await
            .or_throw(&ctx)?;
        Ok(())
    }

    async fn prepare(&self, ctx: Ctx<'_>, sql: String) -> Result<Statement> {
        let stmt = sqlx::Statement::to_owned(&self.pool.prepare(&sql).await.or_throw(&ctx)?);
        Ok(Statement::new(stmt, self.pool.clone()))
    }

    async fn close(&mut self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rquickjs::CatchResultExt;
    use rquickjs_extra_test::{call_test, test_async_with, ModuleEvaluator};

    use crate::SqliteModule;

    #[tokio::test]
    async fn test_database_exec() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<SqliteModule>(ctx.clone(), "sqlite")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { open } from "sqlite";

                        export async function test() {
                            const db = await open({ inMemory: true });
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

    #[tokio::test]
    async fn test_database_close() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<SqliteModule>(ctx.clone(), "sqlite")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { open } from "sqlite";

                        export async function test() {
                            const db = await open({ inMemory: true });
                            await db.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT);");
                            await db.close();
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
