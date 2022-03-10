use std::fs;
use std::io;

#[derive(Debug)]
enum LetterStatus<'a> {
    Correct(&'a str, usize),
    Close(&'a str),
    NotFound(&'a str),
}

fn get_pairs(word: &str) -> Vec<(&str, &str)> {
    let letters: Vec<&str> = word.split_whitespace().collect();

    let pairs: Vec<(&str, &str)> = letters
        .into_iter()
        .map(|pair| {
            let current_pair: Vec<&str> = pair.split('-').collect();
            (current_pair[0], current_pair[1])
        })
        .collect();

    pairs
}

// TODO(Hícaro): Big and unclear, variable names aren't that good
fn tokenize_word(word: &str) -> Vec<LetterStatus> {
    let pairs = get_pairs(word);

    let mut tokens: Vec<LetterStatus> = vec![];

    for (position, pair) in pairs.into_iter().enumerate() {
        let (letter, status) = pair;

        match status {
            "C" => tokens.push(LetterStatus::Correct(letter, position)),
            "Cl" => tokens.push(LetterStatus::Close(letter)),
            "Nf" => tokens.push(LetterStatus::NotFound(letter)),
            token => {
                eprintln!("Unexpected token: {token}");
                std::process::exit(1);
            }
        }
    }
    tokens
}

fn filter_words_with_matches<'a>(tokens: &'a Vec<LetterStatus>, wordle_words: &'a mut Vec<&str>) {
    let mut matches: Vec<&str> = wordle_words.clone();

    for token in tokens.into_iter() {
        matches = match token {
            LetterStatus::Correct(letter, position) => matches
                .into_iter()
                .filter(|word| {
                    word.contains(letter)
                        && word.chars().nth(*position).unwrap() == letter.chars().nth(0).unwrap()
                })
                .collect(),

            LetterStatus::Close(letter) => matches
                .into_iter()
                .filter(|word| word.contains(letter))
                .collect(),

            LetterStatus::NotFound(letter) => matches
                .into_iter()
                .filter(|word| !word.contains(letter))
                .collect(),
        };
    }
    *wordle_words = matches;
}

fn get_word() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected a string");
    input
}

fn main() {
    let wordle_words_file = match fs::read_to_string("datasets/wordle_word_list.txt") {
        Ok(content) => content,

        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let mut wordle_words = wordle_words_file.split('\n').collect();

    let mut i = 5;
    while i != 0 {
        let word = get_word();
        let tokenized_word = tokenize_word(&word);
        filter_words_with_matches(&tokenized_word, &mut wordle_words);

        for word in wordle_words.iter() {
            println!("{word}")
        }

        i -= 1;
    }
}
