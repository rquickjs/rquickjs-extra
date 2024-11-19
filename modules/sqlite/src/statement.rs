use rquickjs::function::Rest;
use rquickjs::{Ctx, Object, Result};
use rquickjs_extra_utils::result::ResultExt;
use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::Sqlite;
use sqlx::{sqlite::SqliteStatement, Column as _, Row as _, SqlitePool, Statement as _};

use super::{Argument, Value};

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct Statement {
    #[qjs(skip_trace)]
    stmt: SqliteStatement<'static>,
    #[qjs(skip_trace)]
    pool: SqlitePool,
}

impl Statement {
    pub fn new(stmt: SqliteStatement<'static>, pool: SqlitePool) -> Self {
        Self { stmt, pool }
    }

    fn query<'js, 'q>(
        &'q self,
        ctx: &Ctx<'js>,
        binds: &'q [Argument<'js>],
    ) -> Result<Query<'q, Sqlite, SqliteArguments<'q>>>
    where
        'js: 'q,
    {
        let mut query = self.stmt.query();
        for value in binds {
            value.try_bind(ctx, &mut query)?;
        }
        Ok(query)
    }

    fn row_to_object<'js>(ctx: &Ctx<'js>, row: &sqlx::sqlite::SqliteRow) -> Result<Object<'js>> {
        let obj = Object::new(ctx.clone())?;
        for column in row.columns() {
            let value = Value::try_read(ctx, column, row)?;
            obj.set(column.name(), value)?;
        }
        Ok(obj)
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl Statement {
    async fn all<'js>(
        &self,
        ctx: Ctx<'js>,
        anon_params: Rest<Argument<'js>>,
    ) -> Result<Vec<Object<'js>>> {
        let query = self.query(&ctx, &anon_params.0)?;

        let rows = query.fetch_all(&self.pool).await.or_throw(&ctx)?;

        let mut res = Vec::with_capacity(rows.len());
        for row in rows {
            let obj = Self::row_to_object(&ctx, &row)?;
            res.push(obj);
        }
        Ok(res)
    }

    async fn get<'js>(
        &self,
        ctx: Ctx<'js>,
        anon_params: Rest<Argument<'js>>,
    ) -> Result<Option<Object<'js>>> {
        let query = self.query(&ctx, &anon_params.0)?;

        let Some(row) = query.fetch_optional(&self.pool).await.or_throw(&ctx)? else {
            return Ok(None);
        };

        let obj = Self::row_to_object(&ctx, &row)?;
        Ok(Some(obj))
    }

    async fn run<'js>(
        &self,
        ctx: Ctx<'js>,
        anon_params: Rest<Argument<'js>>,
    ) -> Result<Object<'js>> {
        let query = self.query(&ctx, &anon_params.0)?;

        let res = query.execute(&self.pool).await.or_throw(&ctx)?;

        let obj = Object::new(ctx.clone())?;
        obj.set("changes", res.rows_affected())?;
        obj.set("lastInsertRowid", res.last_insert_rowid())?;
        Ok(obj)
    }
}

#[cfg(test)]
mod tests {
    use rquickjs::CatchResultExt;
    use rquickjs_extra_test::{call_test, test_async_with, ModuleEvaluator};

    use crate::SqliteModule;

    #[tokio::test]
    async fn test_statement_all() {
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
                            await db.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)");
                            await db.exec("INSERT INTO test (name) VALUES ('test')");
                            await db.exec("INSERT INTO test (name) VALUES ('test2')");
                            const stmt = await db.prepare("SELECT * FROM test");
                            const rows = await stmt.all();
                            return rows[1].id;
                        }
                    "#,
                )
                .await
                .catch(&ctx)
                .unwrap();

                let result = call_test::<i64, _>(&ctx, &module, ()).await;
                assert_eq!(result, 2);
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_statement_get() {
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
                            await db.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)");
                            await db.exec("INSERT INTO test (name) VALUES ('test')");
                            const stmt = await db.prepare("SELECT * FROM test WHERE name = ?");
                            const row = await stmt.get('test');
                            return row.id;
                        }
                    "#,
                )
                .await
                .catch(&ctx)
                .unwrap();

                let result = call_test::<i64, _>(&ctx, &module, ()).await;
                assert_eq!(result, 1);
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_statement_run() {
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
                            await db.exec("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)");
                            const stmt = await db.prepare("INSERT INTO test (name) VALUES (?), (?)");
                            const res = await stmt.run('test', 'test2');
                            return res.changes;
                        }
                    "#,
                )
                .await
                .catch(&ctx)
                .unwrap();

                let result = call_test::<i64, _>(&ctx, &module, ()).await;
                assert_eq!(result, 2);
            })
        })
        .await;
    }
}
