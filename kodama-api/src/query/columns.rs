use super::{IntoValue, Value};

#[derive(Clone, Debug)]
pub enum Column {
    All,
    Name(String),
    Alias(String, String),
    Value(Box<Value>),
    ValueAlias(String, Box<Value>),
}

impl Column {
    /// Write column name to buffer. This will split the name at '.' and
    /// surround each part with backticks.
    pub(crate) fn generate_name(buffer: &mut String, name: &str) {
        for (i, part) in name.split('.').enumerate() {
            if i > 0 {
                buffer.push('.');
            }
            buffer.push('`');
            buffer.push_str(part);
            buffer.push('`');
        }
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        match self {
            Self::All => buffer.push('*'),
            Self::Name(name) => {
                Self::generate_name(buffer, name);
            }
            Self::Alias(name, alias) => {
                Self::generate_name(buffer, name);
                buffer.push_str(" as ");
                Self::generate_name(buffer, alias);
            }
            Self::Value(value) => value.generate(buffer),
            Self::ValueAlias(alias, value) => {
                value.generate(buffer);
                buffer.push_str(" as ");
                Self::generate_name(buffer, alias);
            }
        }
    }
}

pub struct Columns {
    columns: Vec<Column>,
}

impl Columns {
    pub fn list() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    pub fn push(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn all(mut self) -> Self {
        self.columns.push(Column::All);
        self
    }

    pub fn column(mut self, name: &str) -> Self {
        self.columns.push(Column::Name(name.into()));
        self
    }

    pub fn alias(mut self, name: &str, alias: &str) -> Self {
        self.columns.push(Column::Alias(name.into(), alias.into()));
        self
    }

    pub fn count_all(self) -> Self {
        self.value(Value::Count(Box::new(Column::All.into_value())))
    }

    pub fn value(mut self, value: impl IntoValue) -> Self {
        self.columns
            .push(Column::Value(Box::new(value.into_value())));
        self
    }

    pub fn value_as(mut self, alias: &str, value: impl IntoValue) -> Self {
        self.columns.push(Column::ValueAlias(
            alias.into(),
            Box::new(value.into_value()),
        ));
        self
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                buffer.push(',');
            }
            column.generate(buffer);
        }
    }
}

pub trait IntoColumns {
    fn into_columns(self) -> Columns;
}

impl IntoColumns for Columns {
    fn into_columns(self) -> Columns {
        self
    }
}

impl IntoColumns for Column {
    fn into_columns(self) -> Columns {
        Columns {
            columns: vec![self],
        }
    }
}

impl IntoColumns for Value {
    fn into_columns(self) -> Columns {
        Column::Value(Box::new(self)).into_columns()
    }
}

impl<T> std::ops::Add<T> for Column
where
    T: IntoValue,
{
    type Output = Value;

    fn add(self, rhs: T) -> Self::Output {
        self.into_value() + rhs
    }
}

impl<T> std::ops::Sub<T> for Column
where
    T: IntoValue,
{
    type Output = Value;

    fn sub(self, rhs: T) -> Self::Output {
        self.into_value() - rhs
    }
}

impl<T> std::ops::Mul<T> for Column
where
    T: IntoValue,
{
    type Output = Value;

    fn mul(self, rhs: T) -> Self::Output {
        self.into_value() * rhs
    }
}

impl<T> std::ops::Div<T> for Column
where
    T: IntoValue,
{
    type Output = Value;

    fn div(self, rhs: T) -> Self::Output {
        self.into_value() / rhs
    }
}
