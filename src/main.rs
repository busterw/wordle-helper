use colored::Colorize;
use colour::Colour;
use words_db::get_all_words;
use std::io::{self, Write};
mod colour;
mod words_db;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let letters: Vec<Letter> = input
            .split(",")
            .map(|x| Letter {
                letter: x.chars().nth(0).unwrap(),
                colour: x.chars().nth(1).unwrap().into(),
            })
            .collect();

        let words_from_db = get_all_words();

        for word in words_from_db {
            println!("{}", word)
        }
    }
}

pub struct Letter {
    letter: char,
    colour: Colour,
}