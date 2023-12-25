pub mod columns;
pub mod delete;
pub mod insert;
pub mod join;
pub mod select;
pub mod update;
pub mod value;

pub use columns::*;
pub use delete::*;
pub use insert::*;
pub use join::*;
pub use select::*;
pub use update::*;
pub use value::*;

#[derive(Clone, Debug)]
pub struct Query {
    pub(crate) query: String,
}

impl Query {
    pub fn select(columns: impl IntoColumns) -> SelectBeforeFrom {
        Select::columns(columns)
    }

    pub fn select_from(table: &str) -> Select {
        Select::from(table)
    }

    pub fn delete_from(table: &str) -> Delete {
        Delete::from(table)
    }

    pub fn update(table: &str) -> Update {
        Update::table(table)
    }

    pub fn insert_into(table: &str) -> Insert {
        Insert::table(table)
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        buffer.push_str(&self.query);
    }

    pub fn as_string(&self) -> String {
        self.query.clone()
    }
}

pub trait IntoQuery {
    fn into_query(self) -> Query;
}

pub fn coalesce(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Coalesce(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn count(value: impl IntoValue) -> Value {
    Value::Count(Box::new(value.into_value()))
}

pub fn sum(value: impl IntoValue) -> Value {
    Value::Sum(Box::new(value.into_value()))
}

pub fn max(value: impl IntoValue) -> Value {
    Value::Max(Box::new(value.into_value()))
}

pub fn avg(value: impl IntoValue) -> Value {
    Value::Avg(Box::new(value.into_value()))
}

pub fn column(name: &str) -> Column {
    Column::Name(name.into())
}

pub fn param(value: i64) -> Value {
    Value::Param(value)
}

pub fn alias(value: impl IntoValue, alias: &str) -> Column {
    Column::ValueAlias(alias.into(), Box::new(value.into_value()))
}

pub fn is_null(value: impl IntoValue) -> Value {
    Value::IsNull(Box::new(value.into_value()))
}

pub fn is_not_null(value: impl IntoValue) -> Value {
    Value::IsNotNull(Box::new(value.into_value()))
}

pub fn now() -> Value {
    Value::Now
}

pub fn bool_to_integer(value: impl IntoValue) -> Value {
    Value::BoolToInteger(Box::new(value.into_value()))
}

pub fn eq(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Eq(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn not_in(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::NotIn(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn is_in(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::In(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn between(a: impl IntoValue, b: impl IntoValue, c: impl IntoValue) -> Value {
    let a = a.into_value();
    Value::And(
        Box::new(Value::Ge(Box::new(a.clone()), Box::new(b.into_value()))),
        Box::new(Value::Lt(Box::new(a.clone()), Box::new(c.into_value()))),
    )
}

pub fn ne(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Ne(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn ge(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Ge(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn gt(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Gt(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn le(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Le(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn lt(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Lt(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn and(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::And(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn and3(a: impl IntoValue, b: impl IntoValue, c: impl IntoValue) -> Value {
    Value::And(
        Box::new(a.into_value()),
        Box::new(Value::And(
            Box::new(b.into_value()),
            Box::new(c.into_value()),
        )),
    )
}

pub fn or(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Or(Box::new(a.into_value()), Box::new(b.into_value()))
}

pub fn add(a: impl IntoValue, b: impl IntoValue) -> Value {
    Value::Add(Box::new(a.into_value()), Box::new(b.into_value()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let query = Query::select(coalesce(sum(column("amount")), 0))
            .from("Journal")
            .condition(and3(
                eq(column("account_id"), param(1)),
                ge(column("date"), param(2)),
                lt(column("date"), param(3)),
            ));

        assert_eq!(query.into_query().query, "select coalesce(sum(`amount`),0) from `Journal` where (`account_id`=?1 and (`date`>=?2 and `date`<?3))");
    }
}
