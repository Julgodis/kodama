use super::{Column, IntoQuery, IntoValue, Query, Value};

pub struct Update {
    table: String,
    set_clauses: Vec<(String, Value)>,
    where_clauses: Vec<Value>,
}

impl Update {
    pub fn table(table: &str) -> Self {
        Self {
            table: table.into(),
            set_clauses: Vec::new(),
            where_clauses: Vec::new(),
        }
    }

    pub fn set(mut self, column: &str, value: impl IntoValue) -> Self {
        self.set_clauses.push((column.into(), value.into_value()));
        self
    }

    pub fn condition(mut self, value: impl IntoValue) -> Self {
        self.where_clauses.push(value.into_value());
        self
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        buffer.push_str("update ");
        buffer.push('`');
        buffer.push_str(&self.table);
        buffer.push('`');
        buffer.push_str(" set ");
        for (i, (column, value)) in self.set_clauses.iter().enumerate() {
            if i > 0 {
                buffer.push_str(",");
            }
            Column::generate_name(buffer, column);
            buffer.push_str("=");
            value.generate(buffer);
        }
        if !self.where_clauses.is_empty() {
            buffer.push_str(" where ");
            for (i, clause) in self.where_clauses.iter().enumerate() {
                if i > 0 {
                    buffer.push_str(" and ");
                }
                clause.generate(buffer);
            }
        }
    }
}

impl IntoQuery for Update {
    fn into_query(self) -> Query {
        let mut buffer = String::with_capacity(1024);
        self.generate(&mut buffer);
        Query { query: buffer }
    }
}
