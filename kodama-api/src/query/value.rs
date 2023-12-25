use super::{Column, IntoQuery, Query};

#[derive(Clone, Debug)]
pub enum Value {
    Sum(Box<Value>),
    Max(Box<Value>),
    Avg(Box<Value>),
    Coalesce(Box<Value>, Box<Value>),
    Column(Column),
    Param(i64),
    Integer(i64),
    Double(f64),
    String(String),
    Count(Box<Value>),
    As(Box<Value>, String),
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Div(Box<Value>, Box<Value>),
    And(Box<Value>, Box<Value>),
    Or(Box<Value>, Box<Value>),
    Eq(Box<Value>, Box<Value>),
    Ne(Box<Value>, Box<Value>),
    Ge(Box<Value>, Box<Value>),
    Gt(Box<Value>, Box<Value>),
    Le(Box<Value>, Box<Value>),
    Lt(Box<Value>, Box<Value>),
    NotIn(Box<Value>, Box<Value>),
    In(Box<Value>, Box<Value>),
    InlineQuery(Query),
    IsNull(Box<Value>),
    IsNotNull(Box<Value>),
    BoolToInteger(Box<Value>),
    Random,
    Now,
}

impl Value {
    pub(crate) fn generate(&self, buffer: &mut String) {
        match self {
            Self::Sum(field) => {
                buffer.push_str("sum(");
                field.generate(buffer);
                buffer.push(')');
            }
            Self::Max(field) => {
                buffer.push_str("max(");
                field.generate(buffer);
                buffer.push(')');
            }
            Self::Avg(field) => {
                buffer.push_str("avg(");
                field.generate(buffer);
                buffer.push(')');
            }
            Self::Coalesce(a, b) => {
                buffer.push_str("coalesce(");
                a.generate(buffer);
                buffer.push(',');
                b.generate(buffer);
                buffer.push(')');
            }
            Self::Column(column) => {
                column.generate(buffer);
            }
            Self::Param(value) => {
                buffer.push('?');
                buffer.push_str(&value.to_string());
            }
            Self::Integer(value) => {
                buffer.push_str(&value.to_string());
            }
            Self::Double(value) => {
                buffer.push_str(&value.to_string());
            }
            Self::String(value) => {
                buffer.push('\'');
                buffer.push_str(value);
                buffer.push('\'');
            }
            Self::Count(field) => {
                buffer.push_str("count(");
                field.generate(buffer);
                buffer.push(')');
            }
            Self::As(field, alias) => {
                field.generate(buffer);
                buffer.push_str(" as ");
                buffer.push('`');
                buffer.push_str(alias);
                buffer.push('`');
            }
            Self::Add(a, b) => {
                buffer.push('(');
                a.generate(buffer);
                buffer.push_str("+");
                b.generate(buffer);
                buffer.push(')');
            }
            Self::Sub(a, b) => {
                buffer.push('(');
                a.generate(buffer);
                buffer.push_str("-");
                b.generate(buffer);
                buffer.push(')');
            }
            Self::Mul(a, b) => {
                buffer.push('(');
                a.generate(buffer);
                buffer.push_str("*");
                b.generate(buffer);
                buffer.push(')');
            }
            Self::Div(a, b) => {
                buffer.push('(');
                a.generate(buffer);
                buffer.push_str("/");
                b.generate(buffer);
                buffer.push(')');
            }
            Self::And(a, b) => {
                buffer.push('(');
                a.generate(buffer);
                buffer.push_str(" and ");
                b.generate(buffer);
                buffer.push(')');
            }
            Self::Or(a, b) => {
                buffer.push('(');
                a.generate(buffer);
                buffer.push_str(" or ");
                b.generate(buffer);
                buffer.push(')');
            }
            Self::Eq(a, b) => {
                a.generate(buffer);
                buffer.push_str("=");
                b.generate(buffer);
            }
            Self::Ne(a, b) => {
                a.generate(buffer);
                buffer.push_str("<>");
                b.generate(buffer);
            }
            Self::Ge(a, b) => {
                a.generate(buffer);
                buffer.push_str(">=");
                b.generate(buffer);
            }
            Self::Gt(a, b) => {
                a.generate(buffer);
                buffer.push_str(">");
                b.generate(buffer);
            }
            Self::Le(a, b) => {
                a.generate(buffer);
                buffer.push_str("<=");
                b.generate(buffer);
            }
            Self::Lt(a, b) => {
                a.generate(buffer);
                buffer.push_str("<");
                b.generate(buffer);
            }
            Self::NotIn(a, b) => {
                a.generate(buffer);
                buffer.push_str(" not in ");
                b.generate(buffer);
            }
            Self::In(a, b) => {
                a.generate(buffer);
                buffer.push_str(" in ");
                b.generate(buffer);
            }
            Self::InlineQuery(query) => {
                buffer.push('(');
                query.generate(buffer);
                buffer.push(')');
            }
            Self::IsNull(value) => {
                value.generate(buffer);
                buffer.push_str(" is null");
            }
            Self::IsNotNull(value) => {
                value.generate(buffer);
                buffer.push_str(" is not null");
            }
            Self::BoolToInteger(value) => {
                buffer.push_str("(case when ");
                value.generate(buffer);
                buffer.push_str(" then 1 else 0 end)");
            }
            Self::Random => {
                buffer.push_str("random()");
            }
            Self::Now => {
                buffer.push_str("datetime('now')");
            }
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Column {
    fn into_value(self) -> Value {
        Value::Column(self)
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Integer(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Double(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::String(self.into())
    }
}

impl<T> IntoValue for T
where
    T: IntoQuery,
{
    fn into_value(self) -> Value {
        Value::InlineQuery(self.into_query())
    }
}

impl<T> std::ops::Add<T> for Value
where
    T: IntoValue,
{
    type Output = Value;

    fn add(self, rhs: T) -> Self::Output {
        Value::Add(Box::new(self), Box::new(rhs.into_value()))
    }
}

impl<T> std::ops::Sub<T> for Value
where
    T: IntoValue,
{
    type Output = Value;

    fn sub(self, rhs: T) -> Self::Output {
        Value::Sub(Box::new(self), Box::new(rhs.into_value()))
    }
}

impl<T> std::ops::Mul<T> for Value
where
    T: IntoValue,
{
    type Output = Value;

    fn mul(self, rhs: T) -> Self::Output {
        Value::Mul(Box::new(self), Box::new(rhs.into_value()))
    }
}

impl<T> std::ops::Div<T> for Value
where
    T: IntoValue,
{
    type Output = Value;

    fn div(self, rhs: T) -> Self::Output {
        Value::Div(Box::new(self), Box::new(rhs.into_value()))
    }
}
