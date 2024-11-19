use rquickjs::{Ctx, Exception, IntoJs, Result, String, TypedArray};
use rquickjs_extra_utils::result::ResultExt;
use sqlx::sqlite::{SqliteColumn, SqliteRow};
use sqlx::{Column as _, Decode, Row as _, TypeInfo as _, ValueRef};

pub enum Value<'q> {
    Null,
    Integer(i64),
    Real(f64),
    Text(&'q str),
    Blob(&'q [u8]),
}

impl<'q, 'js> IntoJs<'js> for Value<'q>
where
    'js: 'q,
{
    fn into_js(self, ctx: &Ctx<'js>) -> Result<rquickjs::Value<'js>> {
        match self {
            Value::Null => Ok(rquickjs::Value::new_null(ctx.clone())),
            Value::Integer(int) => Ok(int.into_js(ctx)?),
            Value::Real(float) => Ok(float.into_js(ctx)?),
            Value::Text(s) => Ok(String::from_str(ctx.clone(), s)?.into_value()),
            Value::Blob(b) => Ok(TypedArray::<u8>::new_copy(ctx.clone(), b)?.into_value()),
        }
    }
}

impl<'q> Value<'q> {
    pub fn try_read(ctx: &Ctx<'_>, column: &'q SqliteColumn, row: &'q SqliteRow) -> Result<Self> {
        let value = row.try_get_raw(column.ordinal()).or_throw(ctx)?;

        // This is annoying since in theory sqlx can change the string representation
        // but we don't really have a better way to get that information.
        // Also note that only base types will be present in that call, if we want to
        // get more fancy (boolean, datetime, etc) we need to use the column type info.
        // See https://github.com/launchbadge/sqlx/issues/606
        match value.type_info().name() {
            "NULL" => Ok(Value::Null),
            "INTEGER" => Ok(Value::Integer(Decode::decode(value).or_throw(ctx)?)),
            "REAL" => Ok(Value::Real(Decode::decode(value).or_throw(ctx)?)),
            "TEXT" => Ok(Value::Text(Decode::decode(value).or_throw(ctx)?)),
            "BLOB" => Ok(Value::Blob(Decode::decode(value).or_throw(ctx)?)),
            name => Err(Exception::throw_message(
                ctx,
                &["Unsupported type: ", name].concat(),
            )),
        }
    }
}
