use rusqlite::{params, Connection, Error, Result};
use std::{env, path::PathBuf};

use crate::Letter;

pub fn get_all_words() -> Vec<String> {
    let mut db_path = env::current_dir().unwrap();
    db_path.push("words.db3");

    let conn = Connection::open(db_path).unwrap();

    let mut stmt = conn.prepare("SELECT word FROM words").unwrap();

    let rows = stmt
        .query_map([], |row| {
            let word: String = row.get(0)?;
            Ok(word)
        })
        .unwrap();

    let mut words = Vec::new();

    for word_result in rows {
        words.push(word_result.unwrap());
    }

    words
}
