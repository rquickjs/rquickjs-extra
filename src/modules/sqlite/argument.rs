use rquickjs::{Ctx, Exception, FromJs, Result, TypedArray, Value};

use crate::ffi::{CString, CVec};
use crate::utils::result::ResultExt;

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
