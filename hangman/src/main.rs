extern crate unicode_segmentation;
use clearscreen;
use std::cmp;
use std::fmt;
use std::io;
use std::io::Write;
use std::{thread, time};
use unicode_segmentation::UnicodeSegmentation;

mod words;

struct GameState {
    lives: u32,
    score: u32,
    word_graphemes: Vec<Option<String>>,
    guess_graphemes: Vec<Option<String>>,
}

impl GameState {
    fn new() -> GameState {
        let mut new_state = GameState {
            lives: 0,
            score: 0,
            word_graphemes: vec![None; 0],
            guess_graphemes: vec![None; 0],
        };
        new_state.next_word();
        return new_state;
    }

    fn next_word(&mut self) {
        self.word_graphemes = words::get_random_word();
        self.guess_graphemes = vec![None; self.word_graphemes.len()];
        self.lives = cmp::max(self.lives, (self.word_graphemes.len() as u32) / 2);
    }

    fn reset(&mut self) {
        *self = GameState::new();
    }

    fn input_guess(&mut self, guessed_grapheme: &str) {
        assert_eq!(self.lives > 0, true);
        assert_eq!(
            self.guess_graphemes.len() == self.word_graphemes.len(),
            true
        );

        let mut complete = true;
        let mut matched = false;

        for (idx, grapheme) in self.word_graphemes.iter().enumerate() {
            if let Some(grapheme_value) = grapheme {
                if let Some(_) = self.guess_graphemes[idx] {
                    continue;
                }
                if *grapheme_value == guessed_grapheme {
                    self.guess_graphemes[idx] = Some(guessed_grapheme.to_owned());
                    matched = true;
                    continue;
                }
                complete = false;
            }
        }

        if complete {
            println!("{}", &self);
            println!("✅");
            thread::sleep(time::Duration::from_secs(2));
            self.score += 1;
            self.next_word();
            return;
        }

        if !matched {
            self.lives -= 1;
            println!("{}", &self);
            println!("😳");
            if self.lives == 0 {
                self.reset();
                thread::sleep(time::Duration::from_secs(1));
            }
            thread::sleep(time::Duration::from_millis(500));
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        clearscreen::clear().ok();
        write!(f, "\nLives: ")?;

        for _ in 0..self.lives {
            write!(f, "💖")?;
        }

        writeln!(f, "\nScore: {}", self.score)?;

        writeln!(f, "")?;

        for grapheme in self.guess_graphemes.iter() {
            if let Some(value) = grapheme {
                write!(f, "{} ", value)?;
            } else {
                write!(f, "  ")?;
            }
        }
        writeln!(f, "")?;
        for grapheme in self.word_graphemes.iter() {
            if let Some(_) = grapheme {
                write!(f, "▔ ")?;
            } else {
                write!(f, "  ")?;
            }
        }

        writeln!(f, "")
    }
}

// Loops until valid input is received...
fn get_grapheme_input() -> String {
    return loop {
        print!("❓: ");
        io::stdout().flush().ok().unwrap();
        let mut line: String = String::new();
        if let Err(_) = io::stdin().read_line(&mut line) {
            continue;
        };

        if let Ok(value) = line.to_uppercase().trim().parse() {
            line = value;
        } else {
            continue;
        }

        let graphemes = UnicodeSegmentation::graphemes(&line[..], true).collect::<Vec<&str>>();

        if graphemes.len() != 1 {
            continue;
        }

        break line;
    };
}

fn main() {
    let mut game_state = GameState::new();
    loop {
        println!("{}", &game_state);
        game_state.input_guess(&get_grapheme_input());
    }
}
