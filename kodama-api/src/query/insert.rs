use super::{Column, IntoQuery, IntoValue, Query, Value};

pub struct Insert {
    table: String,
    columns: Vec<String>,
    values: Vec<Value>,
    select: Option<Query>,
    ignore: bool,
}

impl Insert {
    pub fn table(table: &str) -> Self {
        Self {
            table: table.into(),
            columns: Vec::new(),
            values: Vec::new(),
            select: None,
            ignore: false,
        }
    }

    pub fn or_ignore(mut self) -> Self {
        self.ignore = true;
        self
    }

    pub fn column(mut self, column: &str, value: impl IntoValue) -> Self {
        self.columns.push(column.into());
        self.values.push(value.into_value());
        self
    }

    pub fn column_name(mut self, column: &str) -> Self {
        self.columns.push(column.into());
        self
    }

    pub fn select(mut self, select: impl IntoQuery) -> Self {
        let query = select.into_query();
        self.select = Some(query);
        self
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        buffer.push_str("insert ");
        if self.ignore {
            buffer.push_str("or ignore ");
        }
        buffer.push_str("into ");
        buffer.push('`');
        buffer.push_str(&self.table);
        buffer.push('`');
        if !self.columns.is_empty() {
            buffer.push_str(" (");
            for (i, column) in self.columns.iter().enumerate() {
                if i > 0 {
                    buffer.push(',');
                }
                Column::generate_name(buffer, column);
            }
            buffer.push_str(") ");
        }

        if let Some(select) = &self.select {
            select.generate(buffer);
        } else if !self.values.is_empty() {
            buffer.push_str("values (");
            for (i, values) in self.values.iter().enumerate() {
                if i > 0 {
                    buffer.push(',');
                }
                values.generate(buffer);
            }
            buffer.push(')');
        }
    }
}

impl IntoQuery for Insert {
    fn into_query(self) -> Query {
        let mut buffer = String::with_capacity(1024);
        self.generate(&mut buffer);
        Query { query: buffer }
    }
}
