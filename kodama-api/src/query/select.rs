use super::{Column, Columns, IntoColumns, IntoQuery, IntoValue, Join, Query, Value};

pub struct SelectBeforeFrom {
    columns: Columns,
}

impl SelectBeforeFrom {
    pub fn from(self, table: &str) -> Select {
        Select {
            columns: self.columns,
            from: table.into(),
            from_as: None,
            where_clauses: Vec::new(),
            joins: Vec::new(),
            order_by: None,
            order_by_asc: true,
            limit: None,
            with: Vec::new(),
        }
    }

    pub fn from_as(self, table: &str, alias: &str) -> Select {
        Select {
            columns: self.columns,
            from: table.into(),
            from_as: Some(alias.into()),
            where_clauses: Vec::new(),
            joins: Vec::new(),
            order_by: None,
            order_by_asc: true,
            limit: None,
            with: Vec::new(),
        }
    }
}

pub struct Select {
    from: String,
    columns: Columns,
    from_as: Option<String>,
    where_clauses: Vec<Value>,
    joins: Vec<Join>,
    order_by: Option<Value>,
    order_by_asc: bool,
    limit: Option<i64>,
    with: Vec<(String, Query)>,
}

impl IntoQuery for Select {
    fn into_query(self) -> Query {
        let mut buffer = String::with_capacity(1024);
        self.generate(&mut buffer);
        Query { query: buffer }
    }
}

impl Select {
    pub fn columns(columns: impl IntoColumns) -> SelectBeforeFrom {
        SelectBeforeFrom {
            columns: columns.into_columns(),
        }
    }

    pub fn from(table: &str) -> Select {
        Select {
            columns: Columns::list(),
            from: table.into(),
            from_as: None,
            where_clauses: Vec::new(),
            joins: Vec::new(),
            order_by: None,
            order_by_asc: true,
            limit: None,
            with: Vec::new(),
        }
    }

    pub fn from_as(table: &str, alias: &str) -> Select {
        Select {
            columns: Columns::list(),
            from: table.into(),
            from_as: Some(alias.into()),
            where_clauses: Vec::new(),
            joins: Vec::new(),
            order_by: None,
            order_by_asc: true,
            limit: None,
            with: Vec::new(),
        }
    }

    pub fn column(mut self, column: &str) -> Self {
        self.columns.push(Column::Name(column.into()));
        self
    }

    pub fn all_columns(mut self) -> Self {
        self.columns.push(Column::All);
        self
    }

    pub fn column_as(mut self, column: &str, alias: &str) -> Self {
        self.columns
            .push(Column::Alias(column.into(), alias.into()));
        self
    }

    pub fn column_value(mut self, value: impl IntoValue) -> Self {
        self.columns
            .push(Column::Value(Box::new(value.into_value())));
        self
    }

    pub fn condition(mut self, clause: impl IntoValue) -> Self {
        self.where_clauses.push(clause.into_value());
        self
    }

    pub fn where_column(self, column: &str, value: impl IntoValue) -> Self {
        self.condition(crate::query::eq(crate::query::column(column), value))
    }

    pub fn join(mut self, join: Join) -> Self {
        self.joins.push(join);
        self
    }

    pub fn with(mut self, name: &str, query: impl IntoQuery) -> Self {
        self.with.push((name.into(), query.into_query()));
        self
    }

    pub fn order_by_asc(mut self, value: impl IntoValue) -> Self {
        self.order_by = Some(value.into_value());
        self.order_by_asc = true;
        self
    }

    pub fn order_by_desc(mut self, value: impl IntoValue) -> Self {
        self.order_by = Some(value.into_value());
        self.order_by_asc = false;
        self
    }

    pub fn order_by_random(mut self) -> Self {
        self.order_by = Some(Value::Random);
        self.order_by_asc = true;
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub(crate) fn generate(&self, buffer: &mut String) {
        if !self.with.is_empty() {
            buffer.push_str("with ");
            for (i, (name, query)) in self.with.iter().enumerate() {
                if i > 0 {
                    buffer.push(',');
                }
                buffer.push('`');
                buffer.push_str(name);
                buffer.push('`');
                buffer.push_str(" as (");
                query.generate(buffer);
                buffer.push(')');
            }
            buffer.push(' ');
        }

        buffer.push_str("select ");
        self.columns.generate(buffer);
        buffer.push_str(" from ");
        buffer.push('`');
        buffer.push_str(&self.from);
        buffer.push('`');
        if let Some(alias) = &self.from_as {
            buffer.push_str(" as ");
            buffer.push('`');
            buffer.push_str(alias);
            buffer.push('`');
        }
        if !self.joins.is_empty() {
            for join in &self.joins {
                buffer.push(' ');
                join.generate(buffer);
            }
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

        if let Some(order_by) = &self.order_by {
            buffer.push_str(" order by ");
            order_by.generate(buffer);
            if self.order_by_asc {
                buffer.push_str(" asc");
            } else {
                buffer.push_str(" desc");
            }
        }

        if let Some(limit) = self.limit {
            buffer.push_str(" limit ");
            buffer.push_str(&limit.to_string());
        }
    }
}
