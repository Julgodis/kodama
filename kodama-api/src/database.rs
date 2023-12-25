use crate::Result;
use crate::{Client, DatabaseConnection};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

pub struct DatabaseBuilder {
    path: PathBuf,
    kodama: Option<(String, String, SocketAddr)>,
    migrations: Vec<(String, String)>,
}

impl DatabaseBuilder {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            kodama: None,
            migrations: Vec::new(),
        }
    }

    pub fn with_kodama(
        mut self,
        name: impl Into<String>,
        service: impl Into<String>,
        addr: SocketAddr,
    ) -> Self {
        self.kodama = Some((name.into(), service.into(), addr));
        self
    }

    pub fn with_migration(mut self, version: impl Into<String>, sql: impl Into<String>) -> Self {
        self.migrations.push((version.into(), sql.into()));
        self
    }

    pub fn build(self) -> Result<Database> {
        let conn = rusqlite::Connection::open(&self.path)?;
        // enable foreign key constraints
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // create migrations table
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                    version TEXT PRIMARY KEY
                );",
        )?;

        let mut db = Database {
            conn: conn,
            kodama: self
                .kodama
                .map(|(name, service, addr)| Client::from_socketaddr(name, service, addr)),
        };

        db.apply_migrations(&self.migrations)?;
        Ok(db)
    }
}

pub struct Database {
    pub(crate) conn: rusqlite::Connection,
    pub(crate) kodama: Option<Client>,
}

impl Database {
    fn has_migration(&self, version: &str) -> Result<bool> {
        let result = self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM migrations WHERE version = ?1);",
            &[version],
            |row| row.get(0),
        )?;
        Ok(result)
    }

    fn apply_migration(&mut self, version: &str, sql: &str) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.execute("INSERT INTO migrations (version) VALUES (?1)", &[version])?;
        tx.commit()?;
        Ok(())
    }

    fn apply_migrations(&mut self, migrations: &[(String, String)]) -> Result<()> {
        for (version, sql) in migrations {
            if !self.has_migration(version)? {
                tracing::debug!("migration[{}]: apply", version);
                self.apply_migration(version, sql)?;
            }
        }
        Ok(())
    }

    pub fn transaction(&self) -> Result<Transaction<'_>> {
        Transaction::new(self)
    }
}

impl DatabaseConnection for Database {
    fn xprepare(&self, sql: &str) -> rusqlite::Result<rusqlite::Statement<'_>> {
        self.conn.prepare(sql)
    }

    fn xexecute(&self, sql: &str, params: impl rusqlite::Params) -> rusqlite::Result<usize> {
        self.conn.execute(sql, params)
    }

    fn xlast_insert_rowid(&self) -> i64 {
        self.conn.last_insert_rowid()
    }

    fn xchanges(&self) -> i64 {
        self.conn.changes() as i64
    }

    fn kodama_instance(&self) -> Option<&Client> {
        self.kodama.as_ref()
    }
}

pub struct Transaction<'a> {
    conn: &'a rusqlite::Connection,
    kodama: Option<Client>,
    active: bool,
}

impl<'a> Transaction<'a> {
    pub fn new(db: &'a Database) -> Result<Self> {
        let transaction = Self {
            conn: &db.conn,
            kodama: db.kodama.clone(),
            active: true,
        };

        transaction.begin_()?;
        Ok(transaction)
    }

    #[inline]
    fn begin_(&self) -> Result<()> {
        self.conn.execute_batch("BEGIN DEFERRED;")?;
        Ok(())
    }

    pub fn commit(mut self) -> Result<()> {
        self.commit_()
    }

    #[inline]
    fn commit_(&mut self) -> Result<()> {
        if !self.active {
            return Ok(());
        }

        self.conn.execute_batch("COMMIT;")?;
        self.active = false;
        Ok(())
    }

    pub fn rollback(mut self) -> Result<()> {
        self.rollback_()
    }

    #[inline]
    fn rollback_(&mut self) -> Result<()> {
        if !self.active {
            return Ok(());
        }

        self.conn.execute_batch("ROLLBACK;")?;
        self.active = false;
        Ok(())
    }

    #[inline]
    pub fn finish(mut self) -> Result<()> {
        self.finish_()
    }

    fn finish_(&mut self) -> Result<()> {
        self.rollback_()
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        self.finish_().expect("failed to rollback transaction");
    }
}

impl<'a> DatabaseConnection for Transaction<'a> {
    fn xprepare(&self, sql: &str) -> rusqlite::Result<rusqlite::Statement<'_>> {
        self.conn.prepare(sql)
    }

    fn xexecute(&self, sql: &str, params: impl rusqlite::Params) -> rusqlite::Result<usize> {
        self.conn.execute(sql, params)
    }

    fn xlast_insert_rowid(&self) -> i64 {
        self.conn.last_insert_rowid()
    }

    fn xchanges(&self) -> i64 {
        self.conn.changes() as i64
    }

    fn kodama_instance(&self) -> Option<&Client> {
        self.kodama.as_ref()
    }
}
