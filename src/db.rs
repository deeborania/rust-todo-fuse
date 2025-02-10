use std::{
    fs,
    path::{Path, PathBuf},
};

use rusqlite::Connection;

#[derive(Debug)]
pub enum CreateItemError {
    ItemExists,
}

#[derive(Debug)]
pub struct Db {
    path: PathBuf,
    connection: Connection,
}

impl Db {
    pub fn new(path: PathBuf) -> Self {
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
        let sqllite_path = path.join("metadata.db");
        let connection = Connection::open(sqllite_path).unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS files(id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                (),
            )
            .unwrap();
        Db { path, connection }
    }

    pub fn create_item(&mut self, name: &str) -> Result<(), CreateItemError> {
        let tx = self.connection.transaction().unwrap();
        tx.execute("INSERT INTO files(name) VALUES (?1)", [name])
            .unwrap();
        let id = tx.last_insert_rowid();

        let item_path = self.path.join(id.to_string());
        if item_path.exists() {
            return Err(CreateItemError::ItemExists);
        }
        fs::create_dir(item_path).unwrap();

        tx.commit().unwrap();
        Ok(())
    }
}
