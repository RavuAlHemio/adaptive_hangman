use std::collections::HashMap;
use std::cmp::min;

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
        dictionaries.retain(|_, words| words.len() >= hit_count);
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
        count_to_del = min(count_to_del, pats_words.len());
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


#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_dict() -> HangmanDict {
        let mut dict = HangmanDict::new();

        dict.add_word("the");
        dict.add_word("quick");
        dict.add_word("brown");
        dict.add_word("fox");
        dict.add_word("jumps");
        dict.add_word("over");
        dict.add_word("lazy");
        dict.add_word("dog");
        dict.add_word("\u{C5}ngstr\u{F6}m");

        dict
    }

    #[test]
    fn test_add_word() {
        let dict = get_test_dict();

        assert_eq!(dict.patterns_words().len(), 4);

        let three = dict.patterns_words().get("___").unwrap();
        assert_eq!(three.len(), 3);
        assert!(three.iter().any(|w| w == "THE"));
        assert!(three.iter().any(|w| w == "FOX"));
        assert!(three.iter().any(|w| w == "DOG"));

        let four = dict.patterns_words().get("____").unwrap();
        assert_eq!(four.len(), 2);
        assert!(four.iter().any(|w| w == "OVER"));
        assert!(four.iter().any(|w| w == "LAZY"));

        let five = dict.patterns_words().get("_____").unwrap();
        assert_eq!(five.len(), 3);
        assert!(five.iter().any(|w| w == "QUICK"));
        assert!(five.iter().any(|w| w == "BROWN"));
        assert!(five.iter().any(|w| w == "JUMPS"));

        let eight = dict.patterns_words().get("________").unwrap();
        assert_eq!(eight.len(), 1);
        assert!(eight.iter().any(|w| w == "\u{C5}NGSTR\u{D6}M"));
    }

    #[test]
    fn test_remove_below() {
        let mut dict = get_test_dict();
        dict.remove_patterns_below(3);

        assert_eq!(dict.patterns_words().len(), 2);

        let three = dict.patterns_words().get("___").unwrap();
        assert_eq!(three.len(), 3);
        assert!(three.iter().any(|w| w == "THE"));
        assert!(three.iter().any(|w| w == "FOX"));
        assert!(three.iter().any(|w| w == "DOG"));

        let five = dict.patterns_words().get("_____").unwrap();
        assert_eq!(five.len(), 3);
        assert!(five.iter().any(|w| w == "QUICK"));
        assert!(five.iter().any(|w| w == "BROWN"));
        assert!(five.iter().any(|w| w == "JUMPS"));
    }
}
