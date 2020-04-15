use std::collections::{HashMap, HashSet};

use rand::Rng;

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

    pub fn remove_percentage_of_words<R: Rng>(&mut self, percentage: f64, rng: &mut R) {
        let mut pats_words: Vec<(String, String)> = Vec::new();
        for (pattern, words) in &self.pattern_to_dict {
            for word in words {
                pats_words.push((pattern.clone(), word.clone()));
            }
        }

        // shuffle
        for i in 0..pats_words.len()-2 {
            let j = rng.gen_range(0, pats_words.len());
            pats_words.swap(i, j);
        }

        let mut count_to_del = (percentage * (pats_words.len() as f64) / 100.0).floor() as usize;
        if count_to_del > pats_words.len() {
            count_to_del = pats_words.len();
        }
        pats_words.truncate(pats_words.len() - count_to_del);

        self.pattern_to_dict.clear();
        for (pat, word) in pats_words {
            self.pattern_to_dict
                .entry(pat)
                .or_insert_with(Vec::new)
                .push(word);
        }
    }
}

pub fn is_char_guessable(character: char) -> bool {
    character.is_alphabetic()
}
