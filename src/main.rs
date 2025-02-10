use std::path::Path;
use std::path::PathBuf;

use db::Db;

mod db;

fn main() {
    let db_path = std::env::args().nth(1).unwrap();
    let mut db = Db::new(db_path.into());
    db.create_item("test").unwrap();
    db.create_item("test2").unwrap();

    println!("{:?}", db);
}
