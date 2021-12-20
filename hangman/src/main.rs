extern crate unicode_segmentation;
use std::io;
use unicode_segmentation::UnicodeSegmentation;

const LIVES_COUNT: u32 = 10;
const EMPTY_GRAPHEME: &str = "\0";

mod words;

#[derive(Debug)]
struct GameState {
    lives: u32,
    word_graphemes: Vec<String>,
    guess_graphemes: Vec<String>,
}

impl GameState {
    fn new() -> GameState {
        let word_graphemes = words::get_random_word();
        let mut guess_graphemes: Vec<String> = Vec::new();
        for _ in &word_graphemes {
            guess_graphemes.push(EMPTY_GRAPHEME.to_owned());
        }

        let state = GameState {
            lives: LIVES_COUNT,
            word_graphemes,
            guess_graphemes,
        };

        return state;
    }

    fn guess(&mut self, guessed_grapheme: String) {
        println!("Guessing {}", guessed_grapheme);
        assert_eq!(self.lives > 0, true);
        assert_eq!(
            self.guess_graphemes.len() == self.word_graphemes.len(),
            true
        );

        let mut missed = true;

        for (idx, grapheme) in self.word_graphemes.iter().enumerate() {
            if self.guess_graphemes[idx] != EMPTY_GRAPHEME {
                continue;
            }
            if *grapheme == guessed_grapheme {
                self.guess_graphemes[idx] = guessed_grapheme.clone();
                missed = false;
            }
        }

        if missed {
            println!("Missed!");
            self.lives -= 1;
        }
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
        dbg!("{:!}", &game_state);
    }
}
