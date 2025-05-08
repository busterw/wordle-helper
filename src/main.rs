use colour::Colour;
use std::io::{self, Write};
use words_db::get_all_words;
mod colour;
mod words_db;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let guess_letters: Vec<LetterColour> = input
            .split(",")
            .map(|x| LetterColour {
                letter: x.chars().nth(0).unwrap(),
                colour: x.chars().nth(1).unwrap().into(),
            })
            .collect();

        let matching_words = get_matching_words(&guess_letters);

        for word in matching_words {
            println!("{}", word)
        }
    }
}

pub fn get_matching_words(guess: &Vec<LetterColour>) -> Vec<String> {
    let all_words = get_all_words();
    let mut results = Vec::new();

    for word in all_words {
        if word_matches(&word, guess) {
            results.push(word);
        }
    }

    results
}

fn word_matches(word: &str, guess: &Vec<LetterColour>) -> bool {
    let chars: Vec<char> = word.chars().collect();

    for (i, letter) in guess.iter().enumerate() {
        match letter.colour {
            Colour::Green => {
                if chars[i] != letter.letter {
                    return false;
                }
            }
            _ => {}
        }
    }

    for (i, letter) in guess.iter().enumerate() {
        if let Colour::Yellow = letter.colour {
            if chars[i] == letter.letter || !chars.contains(&letter.letter) {
                return false;
            }
        }
    }

    for (i, letter) in guess.iter().enumerate() {
        if let Colour::White = letter.colour {
            let is_used_elsewhere = guess.iter().enumerate().any(|(j, other)| {
                j != i
                    && (other.colour == Colour::Green || other.colour == Colour::Yellow)
                    && other.letter == letter.letter
            });

            if !is_used_elsewhere && chars.contains(&letter.letter) {
                return false;
            }
        }
    }

    true
}

pub struct LetterColour {
    letter: char,
    colour: Colour,
}
