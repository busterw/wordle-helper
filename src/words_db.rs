use rusqlite::{params, Connection, Error, Result};
use std::{env, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use crate::LetterColour;

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

pub fn populate_db_from_file(file_path: &str) -> Result<()> {
    let mut db_path = env::current_dir().unwrap();
    db_path.push("words.db3");

    let mut conn = Connection::open(db_path)?;

    conn.execute("DELETE FROM words", [])?;

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let tx = conn.transaction()?; // use transaction for speed

    for line in reader.lines() {
        let word = line.unwrap().trim().to_lowercase();

        if word.len() == 5 && word.chars().all(|c| c.is_ascii_alphabetic()) {
            tx.execute("INSERT OR IGNORE INTO words (word) VALUES (?)", [&word])?;
        }
    }

    tx.commit()?;
    Ok(())
}


