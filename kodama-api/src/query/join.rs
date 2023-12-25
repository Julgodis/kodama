use super::{IntoValue, Value, Column};

pub enum JoinKind {
    Join,
    Left,
}

pub struct Join {
    kind: JoinKind,
    table: String,
    alias: Option<String>,
    on: Vec<Value>,
}

impl Join {
    pub fn left_as(table: &str, alias: &str) -> Self {
        Self {
            kind: JoinKind::Left,
            table: table.into(),
            alias: Some(alias.into()),
            on: Vec::new(),
        }
    }

    pub fn join(table: &str, alias: &str) -> Self {
        Self {
            kind: JoinKind::Join,
            table: table.into(),
            alias: Some(alias.into()),
            on: Vec::new(),
        }
    }

    pub fn inner(table: &str, alias: &str) -> Self {
        Self {
            kind: JoinKind::Join,
            table: table.into(),
            alias: Some(alias.into()),
            on: Vec::new(),
        }
    }

    pub fn on(mut self, value: impl IntoValue) -> Self {
        self.on.push(value.into_value());
        self
    }

    pub fn on_column(mut self, column: &str, value: impl IntoValue) -> Self {
        let column = Column::Name(column.to_string());
        let value = value.into_value();
        self.on.push(Value::Eq(Box::new(column.into_value()), Box::new(value)));
        self
    }

    pub fn on_match(mut self, column: &str, other: &str) -> Self {
        let column = Column::Name(column.to_string());
        let other = Column::Name(other.to_string());
        self.on.push(Value::Eq(Box::new(column.into_value()), Box::new(other.into_value())));
        self
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        match self.kind {
            JoinKind::Join => buffer.push_str("join "),
            JoinKind::Left => buffer.push_str("left join "),
        }
        buffer.push('`');
        buffer.push_str(&self.table);
        buffer.push('`');
        if let Some(alias) = &self.alias {
            buffer.push_str(" as ");
            buffer.push('`');
            buffer.push_str(&alias);
            buffer.push('`');
        }
        buffer.push_str(" on ");
        for (i, clause) in self.on.iter().enumerate() {
            if i > 0 {
                buffer.push_str(" and ");
            }
            clause.generate(buffer);
        }
    }
}
