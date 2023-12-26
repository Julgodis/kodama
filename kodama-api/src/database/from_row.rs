pub trait FromRow: Sized {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self>;
}

impl FromRow for f64 {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        row.get(0)
    }
}

impl FromRow for i64 {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        row.get(0)
    }
}

impl<A, B> FromRow for (A, B)
where
    A: rusqlite::types::FromSql,
    B: rusqlite::types::FromSql,
{
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok((row.get(0)?, row.get(1)?))
    }
}

impl<A, B, C> FromRow for (A, B, C)
where
    A: rusqlite::types::FromSql,
    B: rusqlite::types::FromSql,
    C: rusqlite::types::FromSql,
{
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    }
}

impl FromRow for () {
    fn from_row(_row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(())
    }
}

impl<T> FromRow for Option<T>
where
    T: rusqlite::types::FromSql,
{
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        row.get(0)
    }
}
