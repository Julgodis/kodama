use rusqlite::{ToSql, types::{ToSqlOutput, self}};
use uuid::Uuid;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Value {
    Uuid(Uuid),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Undefined,
}

impl ToSql for Value {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            Value::Uuid(u) => u.to_sql(),
            Value::String(s) => Ok(ToSqlOutput::Owned(types::Value::Text(s.clone()))),
            Value::Integer(i) => Ok(ToSqlOutput::Owned(types::Value::Integer(*i))),
            Value::Float(f) => Ok(ToSqlOutput::Owned(types::Value::Real(*f))),
            Value::Boolean(b) => Ok(ToSqlOutput::Owned(types::Value::Integer(if *b { 1 } else { 0 }))),
            Value::Null | Value::Undefined => Ok(ToSqlOutput::Owned(types::Value::Null)),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Column {
    pub name: String,
    pub decl_type: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Param {
    pub name: String,
    pub value: Value,
}
