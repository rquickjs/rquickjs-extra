use rquickjs::{Ctx, Exception, FromJs, Result, TypedArray};
use rquickjs_extra_utils::ffi::{CString, CVec};
use rquickjs_extra_utils::result::ResultExt;
use sqlx::Sqlite;
use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;

#[derive(Debug)]
pub enum Argument<'js> {
    Null,
    Integer(i64),
    Real(f64),
    Text(CString<'js>),
    Blob(CVec<'js>),
}

impl<'js> FromJs<'js> for Argument<'js> {
    fn from_js(ctx: &Ctx<'js>, value: rquickjs::Value<'js>) -> Result<Self> {
        if value.is_undefined() || value.is_null() {
            return Ok(Argument::Null);
        } else if let Some(int) = value.as_int() {
            return Ok(Argument::Integer(int as i64));
        } else if let Some(big_int) = value.as_big_int() {
            return Ok(Argument::Integer(big_int.clone().to_i64()?));
        } else if let Some(float) = value.as_float() {
            return Ok(Argument::Real(float));
        } else if let Some(string) = value.as_string() {
            return Ok(Argument::Text(CString::from_string(string.clone())?));
        } else if let Some(object) = value.as_object() {
            if object.as_typed_array::<u8>().is_some() {
                // Lifetime issue: https://github.com/DelSkayn/rquickjs/issues/356
                return Ok(Argument::Blob(CVec::from_array(
                    TypedArray::<u8>::from_value(value.clone()).or_throw(ctx)?,
                )?));
            }
        }
        Err(Exception::throw_type(
            ctx,
            &["Value of type '", value.type_name(), "' is not supported"].concat(),
        ))
    }
}

impl<'js> Argument<'js> {
    pub fn try_bind<'q>(
        &'q self,
        ctx: &Ctx<'js>,
        query: &mut Query<'q, Sqlite, SqliteArguments<'q>>,
    ) -> Result<()>
    where
        'js: 'q,
    {
        match self {
            Argument::Null => query.try_bind::<Option<i32>>(None).or_throw(ctx),
            Argument::Integer(int) => query.try_bind(*int).or_throw(ctx),
            Argument::Real(float) => query.try_bind(*float).or_throw(ctx),
            Argument::Text(string) => query.try_bind(string.as_str()?).or_throw(ctx),
            Argument::Blob(blob) => query.try_bind(blob.as_slice()).or_throw(ctx),
        }
    }
}
