use rquickjs::{Ctx, Exception, FromJs, Result, TypedArray, Value};

use crate::ffi::{CString, CVec};
use crate::utils::result::ResultExt;

/// Sqlite [dynamic type value](http://sqlite.org/datatype3.html). The Value's type is typically
/// dictated by SQLite (not by the caller).
#[derive(Debug)]
pub enum Argument<'js> {
    Null,
    Integer(i64),
    Real(f64),
    Text(CString<'js>),
    Blob(CVec<'js>),
}

/// SQLite data types.
/// See [Fundamental Datatypes](https://sqlite.org/c3ref/c_blob.html).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

impl<'js> Argument<'js> {
    #[inline]
    #[must_use]
    pub fn data_type(&self) -> Type {
        match *self {
            Self::Null => Type::Null,
            Self::Integer(_) => Type::Integer,
            Self::Real(_) => Type::Real,
            Self::Text(_) => Type::Text,
            Self::Blob { .. } => Type::Blob,
        }
    }
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
