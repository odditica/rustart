extern crate unicode_segmentation;
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

const WORD_LIST: &[&'static str] = &[
    "rustacean 🦀",
    "pro gaming 🎮",
    "cool 😎",
    "pizza time 🍕⏰",
    "howdy 🤠",
];

pub fn get_random_word() -> Vec<Option<String>> {
    let index = rand::thread_rng().gen_range(0..WORD_LIST.len());
    let chosen_word: String = WORD_LIST[index].to_uppercase();
    let graphemes = UnicodeSegmentation::graphemes(&chosen_word[..], true).collect::<Vec<&str>>();
    assert_ne!(graphemes.is_empty(), true);

    // Separate into graphemes, replacing whitespace with None
    return graphemes
        .iter()
        .map(|x| {
            if x.trim().is_empty() {
                None
            } else {
                Some(x.to_string())
            }
        })
        .collect();
}
