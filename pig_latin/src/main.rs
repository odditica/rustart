use unicode_segmentation::UnicodeSegmentation;
use std::fmt::Write;

fn pig_latinise(text: &str) -> String {
    // FIXME: word splitting should respect punctuation, ugh
    assert!(!text.is_empty());

    let mut text_buffer = String::new();
    
    for word in text.split_whitespace() {
                
        let first_character = UnicodeSegmentation::graphemes(&word[..], true).next();
        
        if let Some(first_character) = first_character {                                                        
            let first_character_upper = first_character.to_uppercase();                
            if ["A", "E", "I", "O", "U"].contains(&&first_character_upper[..]) {                                
                write!(&mut text_buffer, "{}-hay ", &word).unwrap();    
                continue;
            }            
            if ["B", "C", "D", "F", "G", "J", "K", "L", "M", "N", "P", "Q", "S", "T", "V", "X", "Z"].contains(&&first_character_upper[..]) {                                
                write!(&mut text_buffer, "{}-{}ay ", &word[1..], &first_character).unwrap();    
                continue;
            }                    
            text_buffer.push_str(word);                            
            text_buffer.push_str(" ");        
        }
        else {
            continue;
        }
    }
    
    return text_buffer;
}

fn main() {
    println!("{}", pig_latinise("this is a test sentence, cool"));
}

