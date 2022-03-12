use std::fs;
use std::io;

#[derive(Debug)]
enum Status<'a> {
    Green(&'a str, usize),
    Orange(&'a str),
    Black(&'a str),
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

fn tokenize_word(word: &str) -> Vec<Status> {
    let pairs = get_pairs(word);

    let mut tokens: Vec<Status> = vec![];

    for (position, pair) in pairs.into_iter().enumerate() {
        let (letter, status) = pair;

        match status {
            "C" => tokens.push(Status::Green(letter, position)),
            "Cl" => tokens.push(Status::Orange(letter)),
            "Nf" => tokens.push(Status::Black(letter)),
            token => {
                eprintln!("Unexpected token: {token}");
                std::process::exit(1);
            }
        }
    }
    tokens
}

fn filter_words_with_matches<'a>(tokens: &'a Vec<Status>, wordle_words: &'a mut Vec<&str>) {
    let mut matches: Vec<&str> = wordle_words.clone();

    for token in tokens.into_iter() {
        matches = match token {
            Status::Green(letter, position) => matches
                .into_iter()
                .filter(|word| {
                    word.contains(letter)
                        && word.chars().nth(*position).unwrap() == letter.chars().nth(0).unwrap()
                })
                .collect(),

            Status::Orange(letter) => matches
                .into_iter()
                .filter(|word| word.contains(letter))
                .collect(),

            Status::Black(letter) => matches
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

        println!("---------------");
        for word in wordle_words.iter() {
            println!("{word}");
        }
        println!("---------------\n");

        i -= 1;
    }
}
