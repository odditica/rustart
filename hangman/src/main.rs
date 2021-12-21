extern crate unicode_segmentation;
use std::io;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

const LIVES_COUNT: u32 = 10;
mod words;

#[derive(Debug)]
struct GameState {
    lives: u32,
    word_graphemes: Vec<String>,
    guess_graphemes: Vec<Option<String>>,
}

impl GameState {
    fn new() -> GameState {
        let word_graphemes = words::get_random_word();
        let mut guess_graphemes: Vec<Option<String>> = Vec::new();
        for _ in &word_graphemes {
            guess_graphemes.push(None);
        }

        let state = GameState {
            lives: LIVES_COUNT,
            word_graphemes,
            guess_graphemes,
        };

        return state;
    }

    fn guess(&mut self, guessed_grapheme: String) {        
        assert_eq!(self.lives > 0, true);
        assert_eq!(
            self.guess_graphemes.len() == self.word_graphemes.len(),
            true
        );

        let mut missed = true;

        for (idx, grapheme) in self.word_graphemes.iter().enumerate() {
            if self.guess_graphemes[idx].is_some() {
                continue;
            }
            if *grapheme == guessed_grapheme {
                self.guess_graphemes[idx] = Some(guessed_grapheme.clone());
                missed = false;
            }
        }

        if missed {
            println!("Missed!");
            self.lives -= 1;
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..(self.lives - 1) {
            write!(f, "💘").unwrap();
        }        
        writeln!(f, "").unwrap();

        for grapheme in self.guess_graphemes.iter() {
            if let Some(value) = grapheme {
                write!(f, "{}", value).unwrap();
            } else {
                write!(f, "_").unwrap();
            }
            
        }        

        writeln!(f, "")
    }
}

fn grab_grapheme() -> String {
    return loop {
        println!("Input a character:");
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
        game_state.guess(grab_grapheme());
        println!("{}", &game_state);
    }
}
