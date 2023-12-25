use crate::Client;

pub trait DatabaseConnection {
    fn xprepare(&self, sql: &str) -> rusqlite::Result<rusqlite::Statement<'_>>;
    fn xexecute(&self, sql: &str, params: impl rusqlite::Params) -> rusqlite::Result<usize>;

    fn xlast_insert_rowid(&self) -> i64;
    fn xchanges(&self) -> i64;

    fn kodama_instance(&self) -> Option<&Client>;
    fn kodama<T>(&self, func: impl FnOnce(&Client) -> T) {
        if let Some(kodama) = self.kodama_instance() {
            let _ = func(kodama);
        }
    }
}
