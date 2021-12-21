extern crate unicode_segmentation;
use std::cmp;
use std::fmt;
use std::io;
use std::io::Write;

use clearscreen;
use unicode_segmentation::UnicodeSegmentation;

mod words;

#[derive(Debug)]
struct GameState {
    lives: u32,
    word_graphemes: Vec<Option<String>>,
    guess_graphemes: Vec<Option<String>>,
}

impl GameState {
    fn new() -> GameState {
        let word_graphemes = words::get_random_word();
        let guess_graphemes = vec![None; word_graphemes.len()];
        return GameState {
            lives: cmp::max(10, (word_graphemes.len() as u32) / 2),
            word_graphemes,
            guess_graphemes,
        };
    }

    fn guess(&mut self, guessed_grapheme: String) {
        assert_eq!(self.lives > 0, true);
        assert_eq!(
            self.guess_graphemes.len() == self.word_graphemes.len(),
            true
        );

        let mut missed = true;

        for (idx, grapheme) in self.word_graphemes.iter().enumerate() {
            if let Some(grapheme_value) = grapheme {
                if self.guess_graphemes[idx].is_some() {
                    continue;
                }
                if *grapheme_value == guessed_grapheme {
                    self.guess_graphemes[idx] = Some(guessed_grapheme.clone());
                    missed = false;
                }
            }
        }

        if missed {
            self.lives -= 1;
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nLives: ")?;

        for _ in 0..(self.lives - 1) {
            write!(f, "💖")?;
        }

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

fn grab_grapheme() -> String {
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
        clearscreen::clear().ok();
        println!("{}", &game_state);
        game_state.guess(grab_grapheme());
    }
}
