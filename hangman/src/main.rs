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
    polled_message: Option<(String, time::Duration)>,
}

impl GameState {
    fn new() -> GameState {
        let mut new_state = GameState {
            lives: 0,
            score: 0,
            word_graphemes: vec![None; 0],
            guess_graphemes: vec![None; 0],
            polled_message: None,
        };
        new_state.next_word();
        return new_state;
    }

    // Resets the whole game.
    fn reset(&mut self) {
        *self = GameState::new();
    }

    // Generates a new word to guess.
    fn next_word(&mut self) {
        self.word_graphemes = words::get_random_word();
        self.guess_graphemes = vec![None; self.word_graphemes.len()];
        self.lives = cmp::max(
            self.lives + 4,
            cmp::max(8, (self.word_graphemes.len() / 2) as u32),
        );
    }

    // Queues a message to be displayed the next time we... display.
    fn queue_message(&mut self, message: &str, duration: time::Duration) {
        assert!(message.len() > 0);
        assert_eq!(self.polled_message, None);
        assert_eq!(duration.is_zero(), false);
        self.polled_message = Some((message.to_owned(), duration));
    }

    fn reveal_word(&mut self) {
        self.guess_graphemes = self.word_graphemes.to_owned();
    }

    fn display(&mut self) {
        loop {
            clearscreen::clear().ok();
            println!("{}", &self);

            // If there's a message, display it, wait, then hide it.
            // Perhaps overkill, but could be easily used for an actual message queue.
            if let Some((_, duration)) = &self.polled_message {
                thread::sleep(*duration);
                self.polled_message = None;
                continue;
            }

            break;
        }
    }

    fn logic(&mut self) {
        // Display and ask for guess input.
        self.display();
        let guessed_grapheme = get_grapheme_input();

        // Evaluate the guess.
        let mut complete = true;
        let mut matched = false;

        assert_eq!(self.guess_graphemes.len(), self.word_graphemes.len());
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
            self.score = self.score.saturating_add(self.guess_graphemes.len() as u32);
            self.queue_message("🏆 You got it!", time::Duration::from_secs(2));
            self.display();
            self.next_word();
            return;
        }

        if !matched {
            assert!((self.lives > 0) && !complete);
            self.lives -= 1;
            if self.lives == 0 {
                self.reveal_word();
                self.queue_message("💔 Better luck next time!", time::Duration::from_secs(2));
                self.display();
                self.reset();
            } else {
                self.queue_message("😳", time::Duration::from_millis(500));
            }
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        if let Some((message, _)) = &self.polled_message {
            write!(f, "\n\n{}", message)?;
        }

        write!(f, "")
    }
}

// Loops until valid input is received...
fn get_grapheme_input() -> String {
    println!("");
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
        game_state.logic();
    }
}
