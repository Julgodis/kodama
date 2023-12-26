use crate::{
    query::{IntoQuery, Query},
    DatabaseConnection, FromRow,
};

pub trait DatabaseQuery {
    fn select_one<T>(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<T>
    where
        T: FromRow,
    {
        let result = self.select_maybe(connection, params)?;
        match result {
            Some(value) => Ok(value),
            None => Err(rusqlite::Error::QueryReturnedNoRows),
        }
    }

    fn select_maybe<T>(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Option<T>>
    where
        T: FromRow;

    fn select_many<T>(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Vec<T>>
    where
        T: FromRow;

    fn execute(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()>;

    fn delete(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()> {
        self.execute(connection, params)
    }

    fn update(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()> {
        self.execute(connection, params)
    }

    fn insert(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<i64>;

    fn exists(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<bool>;
}

fn perform<'x, Output, Database, Params>(
    query: &Query,
    connection: &'x Database,
    params: Params,
    func: impl FnOnce(&'x Database, Params, &Query) -> rusqlite::Result<Output>,
) -> rusqlite::Result<Output>
where
    Database: DatabaseConnection,
    Params: rusqlite::Params,
{
    tracing::debug!("query: {:?}", query.query);
    let start = std::time::Instant::now();
    let result = func(connection, params, query);
    let elapsed = start.elapsed();
    let elapsed_us = elapsed.as_micros() as u64;

    tracing::trace!("  execution time: {:?} us", elapsed_us);

    let error: bool = result.is_err();
    connection
        .kodama(|kodama| kodama.record_with_error("sql-query", &query.query, elapsed_us, error));

    result
}

impl DatabaseQuery for Query {
    fn select_maybe<T>(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Option<T>>
    where
        T: FromRow,
    {
        perform(
            self,
            connection,
            params,
            |connection, params, query| -> rusqlite::Result<Option<T>> {
                let mut stmt = connection.xprepare(&query.query)?;
                let result = {
                    let mut rows = stmt.query_map(params, |row| T::from_row(row))?;

                    match rows.next() {
                        Some(Ok(result)) => Ok(Some(result)),
                        Some(Err(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
                        Some(Err(err)) => Err(err),
                        None => Ok(None),
                    }
                };

                if let Some(ext) = stmt.expanded_sql() {
                    tracing::warn!("  expanded sql: {}", ext);
                }

                result
            },
        )
    }

    fn select_many<T>(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Vec<T>>
    where
        T: FromRow,
    {
        perform(
            self,
            connection,
            params,
            |connection, params, query| -> rusqlite::Result<Vec<T>> {
                let mut stmt = connection.xprepare(&query.query)?;
                let result = {
                    let rows = stmt.query_map(params, |row| T::from_row(row))?;
                    

                    rows
                        .into_iter()
                        .inspect(|row| {
                            if let Err(err) = row {
                                tracing::error!("error: {}", err);
                            }
                        })
                        .filter_map(|x| x.ok())
                        .collect::<Vec<_>>()
                };

                if let Some(ext) = stmt.expanded_sql() {
                    tracing::warn!("  expanded sql: {}", ext);
                }

                Ok(result)
            },
        )
    }

    fn execute(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()> {
        perform(
            self,
            connection,
            params,
            |connection, params, query| -> rusqlite::Result<()> {
                let mut stmt = connection.xprepare(&query.query)?;
                stmt.execute(params)?;
                Ok(())
            },
        )?;

        Ok(())
    }

    fn insert(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<i64> {
        let result = perform(
            self,
            connection,
            params,
            |connection, params, query| -> rusqlite::Result<i64> {
                let mut stmt = connection.xprepare(&query.query)?;
                let result = stmt.insert(params)?;
                Ok(result)
            },
        )?;

        Ok(result)
    }

    fn exists(
        &self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<bool> {
        perform(
            self,
            connection,
            params,
            |connection, params, query| -> rusqlite::Result<bool> {
                let mut stmt = connection.xprepare(&query.query)?;
                let mut rows = stmt.query(params)?;
                let result = rows.next()?.is_some();
                Ok(result)
            },
        )
    }
}

pub trait DatabaseQueryShortcut {
    fn select_one<T>(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<T>
    where
        T: FromRow;

    fn select_maybe<T>(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Option<T>>
    where
        T: FromRow;

    fn select_many<T>(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Vec<T>>
    where
        T: FromRow;

    fn delete(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()>;

    fn update(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()>;

    fn insert(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<i64>;

    fn exists(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<bool>;
}

impl<Q> DatabaseQueryShortcut for Q
where
    Q: IntoQuery,
{
    fn select_one<T>(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<T>
    where
        T: FromRow,
    {
        let query = self.into_query();
        DatabaseQuery::select_one(&query, connection, params)
    }

    fn select_maybe<T>(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Option<T>>
    where
        T: FromRow,
    {
        let query = self.into_query();
        DatabaseQuery::select_maybe(&query, connection, params)
    }

    fn select_many<T>(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<Vec<T>>
    where
        T: FromRow,
    {
        let query = self.into_query();
        DatabaseQuery::select_many(&query, connection, params)
    }

    fn delete(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()> {
        let query = self.into_query();
        DatabaseQuery::delete(&query, connection, params)
    }

    fn update(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<()> {
        let query = self.into_query();
        DatabaseQuery::update(&query, connection, params)
    }

    fn insert(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<i64> {
        let query = self.into_query();
        DatabaseQuery::insert(&query, connection, params)
    }

    fn exists(
        self,
        connection: &impl DatabaseConnection,
        params: impl rusqlite::Params,
    ) -> rusqlite::Result<bool> {
        let query = self.into_query();
        DatabaseQuery::exists(&query, connection, params)
    }
}
