use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IOError};

use getopts::Options;

use adaptive_hangman::dict;


fn load_dictionary(file_name: &str) -> Result<dict::HangmanDict, IOError> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut dictionary = dict::HangmanDict::new();

    for line in reader.lines() {
        dictionary.add_word(&line?);
    }

    Ok(dictionary)
}


struct GuessAndWords {
    pub guess: Vec<char>,
    pub words: Vec<String>,
}
impl GuessAndWords {
    pub fn new(guess: Vec<char>, words: Vec<String>) -> GuessAndWords {
        GuessAndWords { guess, words }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "dict", "Specifies the dictionary file to read.", "DICTFILE");
    opts.optflag("h", "help", "Outputs this help.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };
    if matches.opt_present("h") || matches.free.len() != 1 {
        print!("{}", opts.usage(&format!("Usage: {} [options] PATTERN", program)));
        return;
    }

    let pattern = matches.free.get(0).unwrap().clone();
    let dict_path = matches.opt_str("d").unwrap_or_else(|| "dict.txt".to_owned());

    let dict = load_dictionary(&dict_path).unwrap();
    let words_opt: Option<&Vec<String>> = dict.patterns_words().get(&pattern);
    if words_opt.is_none() {
        print!("pattern {} not found in dictionary", pattern);
        return;
    }
    let words: Vec<String> = words_opt.unwrap().clone();

    let mut guesses_words: Vec<GuessAndWords> = Vec::new();
    // start with the empty guess and all words
    guesses_words.push(GuessAndWords::new(
        Vec::new(),
        words,
    ));

    let mut best_guess: Option<Vec<char>> = None;
    let mut best_word: Option<String> = None;

    while !guesses_words.is_empty() {
        // take a gander
        let guess_words = guesses_words.pop().unwrap();

        if guess_words.words.is_empty() {
            // nothing matches :-(
            continue;
        } else if guess_words.words.len() == 1 {
            // exactly one word matches! :-)
            if best_guess.is_none() || best_guess.as_ref().unwrap().len() > guess_words.guess.len() {
                best_guess = Some(guess_words.guess.clone());
                best_word = Some(guess_words.words.get(0).unwrap().clone());
                println!("new best guess: {:?} ({})", best_guess.as_ref().unwrap(), best_word.as_ref().unwrap());
            }
            continue;
        }

        if guess_words.guess.len() > pattern.len() {
            // the guess cannot be longer than the pattern
            continue;
        }
        if best_guess.is_some() && guess_words.guess.len() >= best_guess.as_ref().unwrap().len() {
            // it makes no sense for the next best guess
            // to be longer than the current best guess
            continue;
        }

        // more than one word matches

        // find the guessable letters and in how many remaining words they appear
        let mut letter_wordcount: HashMap<char, usize> = HashMap::new();
        for word in &guess_words.words {
            for c in word.chars() {
                if !dict::is_char_guessable(c) {
                    continue;
                }
                if guess_words.guess.contains(&c) {
                    continue;
                }

                *letter_wordcount.entry(c).or_insert(0) += 1;
            }
        }

        // sort that by the word counts in ascending order
        // (best gets pushed last because guesses_words is LIFO)
        let mut letters_wordcounts: Vec<(char, usize)> = letter_wordcount
            .iter()
            .map(|chco| (*chco.0, *chco.1))
            .collect();
        letters_wordcounts.sort_by_key(|chco| chco.1);

        // try appending each one
        for (ch, _co) in &letters_wordcounts {
            // assemble a new guess
            let mut sub_guess = guess_words.guess.clone();
            sub_guess.push(*ch);

            // assemble the matching word list
            let mut sub_words = Vec::new();
            for word in &guess_words.words {
                if !word.contains(*ch) {
                    sub_words.push(word.clone());
                }
            }

            // enqueue that
            guesses_words.push(GuessAndWords::new(sub_guess, sub_words));
        }
    }

    if best_guess.is_some() {
        println!("guess: {:?}", best_guess.as_ref().unwrap());
        println!("grants word: {:?}", best_word.as_ref().unwrap());
    } else {
        println!("no surgical strike found");
    }
}
