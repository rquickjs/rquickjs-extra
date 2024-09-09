use rquickjs::{Ctx, Exception, FromJs, Result};

use crate::utils::result::ResultExt;

/// Sqlite [dynamic type value](http://sqlite.org/datatype3.html). The Value's type is typically
/// dictated by SQLite (not by the caller).
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
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

impl Value {
    #[inline]
    #[must_use]
    pub fn data_type(&self) -> Type {
        match *self {
            Self::Null => Type::Null,
            Self::Integer(_) => Type::Integer,
            Self::Real(_) => Type::Real,
            Self::Text(_) => Type::Text,
            Self::Blob(_) => Type::Blob,
        }
    }
}

impl<'js> FromJs<'js> for Value {
    fn from_js(ctx: &Ctx<'js>, value: rquickjs::Value<'js>) -> Result<Self> {
        if value.is_undefined() || value.is_null() {
            return Ok(Value::Null);
        } else if let Some(value) = value.as_int() {
            return Ok(Value::Integer(value as i64));
        } else if let Some(value) = value.as_big_int() {
            return Ok(Value::Integer(value.clone().to_i64()?));
        } else if let Some(value) = value.as_float() {
            return Ok(Value::Real(value));
        } else if let Some(value) = value.as_string() {
            return Ok(Value::Text(value.to_string()?));
        } else if let Some(value) = value.as_object() {
            if let Some(value) = value.as_typed_array::<u8>() {
                // FIXME: We should be able to avoid copying the data
                return Ok(Value::Blob(value.as_bytes().or_throw(ctx)?.to_vec()));
            }
        }
        Err(Exception::throw_type(
            ctx,
            &["Value of type '", value.type_name(), "' is not supported"].concat(),
        ))
    }
}
