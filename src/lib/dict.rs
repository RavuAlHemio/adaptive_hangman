use std::collections::{HashMap, HashSet};

pub const PLACEHOLDER_CHAR: char = '_';


pub struct HangmanDict {
    pattern_to_dict: HashMap<String, Vec<String>>,
}


impl HangmanDict {
    pub fn new() -> HangmanDict {
        HangmanDict {
            pattern_to_dict: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        let word_upcase = word.to_uppercase();

        // construct word pattern
        let mut word_pattern = String::new();
        for character in word.chars() {
            let pattern_char = if is_char_guessable(character) {
                PLACEHOLDER_CHAR
            } else {
                character
            };
            word_pattern.push(pattern_char);
        }

        // slap it in there
        self.pattern_to_dict
            .entry(word_pattern)
            .or_insert_with(Vec::new)
            .push(word_upcase);
    }

    pub fn patterns_words(&self) -> &HashMap<String, Vec<String>> {
        &self.pattern_to_dict
    }

    pub fn remove_patterns_below(&mut self, hit_count: usize) {
        let dictionaries = &mut self.pattern_to_dict;
        let mut bad_keys = HashSet::new();

        for (key, val) in dictionaries.iter() {
            if val.len() < hit_count {
                bad_keys.insert(key.clone());
            }
        }

        for bad_key in bad_keys {
            dictionaries.remove(&bad_key);
        }
    }
}

pub fn is_char_guessable(character: char) -> bool {
    character.is_alphabetic()
}
