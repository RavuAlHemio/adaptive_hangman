mod opts;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IOError, stdin};
use std::iter::FromIterator;
use std::process;

use clap::derive::Clap;
use rand::Rng;

use adaptive_hangman::dict;

use crate::opts::Opts;


fn load_dictionary(file_name: &str) -> Result<dict::HangmanDict, IOError> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut dictionary = dict::HangmanDict::new();

    for line in reader.lines() {
        dictionary.add_word(&line?);
    }

    Ok(dictionary)
}


fn output_letter_probabilities(words: &Vec<String>) {
    let mut letter_to_count: HashMap<char, usize> = HashMap::new();
    for word in words {
        let word_as_hash_set: HashSet<char> = HashSet::from_iter(word.chars());
        for c in word_as_hash_set {
            *letter_to_count.entry(c).or_insert(0) += 1;
        }
    }

    let mut letter_and_count: Vec<(char, usize)> = letter_to_count
        .iter()
        .map(|chco| (*chco.0, *chco.1))
        .collect();
    letter_and_count.sort_unstable_by_key(|chco| usize::max_value() - chco.1);

    let letters_counts_str = letter_and_count.iter()
        .map(|chco| format!("{}:{}", chco.0, chco.1))
        .collect::<Vec<String>>()
        .join(", ");
    println!("letter probabilities: {}", letters_counts_str);
}


fn whittle_down_list(words: &Vec<String>, guesses: &Vec<char>) -> Vec<String> {
    // attempt to whittle down the list
    let guess_set: HashSet<char> = HashSet::from_iter(guesses.iter().cloned());
    let mut new_words = Vec::new();
    for word in words {
        let mut use_word = true;
        for c in word.chars() {
            if guess_set.contains(&c) {
                use_word = false;
                break;
            }
        }

        if use_word {
            new_words.push(word.clone());
        }
    }

    new_words
}


struct UpdatedPattern {
    pub pattern: String,
    pub lose_a_life: bool,
    pub won: bool,
}
impl UpdatedPattern {
    pub fn new(pattern: String, lose_a_life: bool, won: bool) -> UpdatedPattern {
        UpdatedPattern { pattern, lose_a_life, won }
    }
}


fn apply_guess_to_pattern(pattern: &str, word: &str, guess: char) -> UpdatedPattern {
    // apply the guess to the pattern
    let word_chars: Vec<char> = word.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let mut new_pattern = String::new();
    let mut lose_a_life = true;
    let mut won = true;

    assert!(word_chars.len() == pattern_chars.len());
    for i in 0..word_chars.len() {
        let pattern_char = *pattern_chars.get(i).unwrap();
        let word_char = *word_chars.get(i).unwrap();
        if pattern_char == dict::PLACEHOLDER_CHAR {
            if word_char == guess {
                lose_a_life = false;
                new_pattern.push(word_char);
            } else {
                won = false;
                new_pattern.push(pattern_char);
            }
        } else {
            new_pattern.push(pattern_char);
        }
    }

    UpdatedPattern::new(new_pattern, lose_a_life, won)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let opts: Opts = match Opts::try_parse_from(args) {
        Ok(o) => o,
        Err(err) => {
            eprint!("{}", err);
            process::exit(1);
        },
    };

    let mut dict = load_dictionary(&opts.dict_path).unwrap();
    if let Some(min_words) = opts.min_words {
        dict.remove_patterns_below(min_words);
    }
    let mut rng = rand::thread_rng();
    if opts.remove_percentage > 0.0 {
        dict.remove_percentage_of_words(opts.remove_percentage, &mut rng);
    }

    if dict.patterns_words().is_empty() {
        println!("no more patterns left");
        return;
    }

    let mut pattern: String = if let Some(pat) = opts.pattern {
        pat
    } else {
        // pick a pattern at random
        let patterns: Vec<&String> = dict.patterns_words().keys().collect();
        let pattern_index = rng.gen_range(0, patterns.len());
        (*patterns.get(pattern_index).unwrap()).clone()
    };
    let mut words: Vec<String> = dict.patterns_words().get(&pattern).unwrap().clone();

    if opts.debug {
        println!("{} words match this pattern", words.len());
    }

    // the fun begins
    let mut word_opt: Option<String> = None;
    let mut guesses: Vec<char> = Vec::new();
    let mut lives = opts.lives;
    while lives > 0 {
        if opts.debug && word_opt.is_none() {
            output_letter_probabilities(&words);
        }

        println!("{} (guessed: {}, lives: {})", pattern, guesses.iter().collect::<String>(), lives);

        println!("Your guess: ");
        let mut line = String::new();
        stdin().lock().read_line(&mut line).unwrap();
        if line.len() == 0 {
            // player gave up?
            break;
        }
        line = line.trim_end().to_uppercase();

        let char_count = line.chars().count();
        if char_count != 1 {
            println!("Sorry, I can only accept one character, not {}!", char_count);
            // let them guess again
            continue;
        }

        let guess = line.chars().nth(0).unwrap();
        if guesses.contains(&guess) {
            println!("You already guessed {}!", guess);
            continue;
        }
        if !dict::is_char_guessable(guess) {
            println!("{} is not a guessable character!", guess);
            continue;
        }
        guesses.push(guess);

        if word_opt.is_none() {
            let new_words = whittle_down_list(&words, &guesses);

            if new_words.len() == 0 {
                if opts.debug {
                    println!("oh no, we can't elude the player any longer");
                }

                // pick a word at random
                let forced_word_index = rng.gen_range(0, words.len());
                let forced_word = words.get(forced_word_index).unwrap();
                word_opt = Some(forced_word.clone());
            } else {
                if opts.debug {
                    println!("ooh, we still have {} word(s)", new_words.len());
                }
                words = new_words;
            }
        }

        if let Some(word) = &mut word_opt {
            let guess_result = apply_guess_to_pattern(&pattern, word, guess);

            pattern = guess_result.pattern;
            if guess_result.won {
                println!("{} -- congratulations!", pattern);
                return;
            } else if guess_result.lose_a_life {
                lives -= 1;
            }
        } else {
            // just subtract a life :-D
            lives -= 1;
        }
    }

    if let Some(fixed_word) = word_opt {
        println!("Sorry! The word was {}!", fixed_word);
    } else {
        let resolution_index = rng.gen_range(0, words.len());
        println!("Sorry! The word was {}!", words.get(resolution_index).unwrap());
        if opts.debug {
            for (i, word) in words.iter().enumerate() {
                if i != resolution_index {
                    println!("or {}", word);
                }
            }
        }
    }
}
