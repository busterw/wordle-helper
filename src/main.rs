use std::io::{self, Write};
use colored::Colorize;
use colour::Colour;
mod colour;

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

        for letter in letters {
            match letter.colour {
                Colour::White => print!("{}",letter.letter.to_string().white()),
                Colour::Yellow => print!("{}",letter.letter.to_string().yellow()),
                Colour::Green => print!("{}",letter.letter.to_string().green()),
            }
        }
    }
}

pub struct Letter {
    letter: char,
    colour: Colour,
}

impl Colour {}
