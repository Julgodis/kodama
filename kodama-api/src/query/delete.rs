use super::{IntoQuery, Query, Value, IntoValue};

pub struct Delete {
    from: String,
    where_clauses: Vec<Value>,
}

impl Delete {
    pub fn from(table: &str) -> Self {
        Self {
            from: table.into(),
            where_clauses: Vec::new(),
        }
    }

    pub fn condition(mut self, value: impl IntoValue) -> Self {
        self.where_clauses.push(value.into_value());
        self
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        buffer.push_str("delete from ");
        buffer.push('`');
        buffer.push_str(&self.from);
        buffer.push('`');
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

impl IntoQuery for Delete {
    fn into_query(self) -> Query {
        let mut buffer = String::with_capacity(1024);
        self.generate(&mut buffer);
        Query { query: buffer }
    }
}
