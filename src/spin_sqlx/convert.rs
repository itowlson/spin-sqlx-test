use std::fmt::Display;

use super::{SqlxConnection, SpinSqliteTypeInfo};

// anyhow::Error makes sqlx mad
#[derive(Debug)]
struct BadTypeError;
impl std::error::Error for BadTypeError {}
impl Display for BadTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("bad type")
    }
}

#[derive(Debug)]
struct BadValError;
impl std::error::Error for BadValError {}
impl Display for BadValError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("bad value")
    }
}




fn into_or_err<T: TryInto<U>, U>(value: T) -> Result<U, sqlx::error::BoxDynError> {
    match value.try_into() {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(BadValError)),
    }
}

impl<'q> sqlx::Encode<'q, SqlxConnection> for &str {
    fn encode_by_ref(&self, buf: &mut <SqlxConnection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Text(self.to_string()));
        sqlx::encode::IsNull::No
    }
}
impl sqlx::Type<SqlxConnection> for &str {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Text
    }
}

impl<'r> sqlx::Decode<'r, SqlxConnection> for String {
    fn decode(value: <SqlxConnection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Text(s) => Ok(s),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<SqlxConnection> for String {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Text
    }
}

impl<'q> sqlx::Encode<'q, SqlxConnection> for u32 {
    fn encode_by_ref(&self, buf: &mut <SqlxConnection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, SqlxConnection> for u32 {
    fn decode(value: <SqlxConnection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<SqlxConnection> for u32 {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, SqlxConnection> for bool {
    fn encode_by_ref(&self, buf: &mut <SqlxConnection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer(if *self { 1 } else { 0 }));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, SqlxConnection> for bool {
    fn decode(value: <SqlxConnection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(0) => Ok(false),
            spin_sdk::sqlite::Value::Integer(1) => Ok(true),
            spin_sdk::sqlite::Value::Integer(_) => Err(Box::new(BadValError)),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<SqlxConnection> for bool {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, SqlxConnection> for f32 {
    fn encode_by_ref(&self, buf: &mut <SqlxConnection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Real((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, SqlxConnection> for f32 {
    fn decode(value: <SqlxConnection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Real(n) => Ok(n as f32),  // TODO: what could go wrong eh
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<SqlxConnection> for f32 {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Real
    }
}

impl<'q> sqlx::Encode<'q, SqlxConnection> for &[u8] {
    fn encode_by_ref(&self, buf: &mut <SqlxConnection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Blob(self.to_vec()));
        sqlx::encode::IsNull::No
    }
}
impl<'q, const N: usize> sqlx::Encode<'q, SqlxConnection> for &[u8; N] {
    fn encode_by_ref(&self, buf: &mut <SqlxConnection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Blob(self.to_vec()));
        sqlx::encode::IsNull::No
    }
}
impl sqlx::Type<SqlxConnection> for &[u8] {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Blob
    }
}
impl<const N: usize> sqlx::Type<SqlxConnection> for &[u8; N] {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Blob
    }
}
impl<'r> sqlx::Decode<'r, SqlxConnection> for Vec<u8> {
    fn decode(value: <SqlxConnection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Blob(v) => Ok(v),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<SqlxConnection> for Vec<u8> {
    fn type_info() -> <SqlxConnection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Blob
    }
}
