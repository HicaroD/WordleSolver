use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Debug)]
enum Status<'a> {
    Green(&'a str, usize),
    Orange(&'a str),
    Black(&'a str),
}

fn is_unique_char(letter: &str, word: &str) -> bool {
    word.matches(letter).count() == 1
}

fn get_pairs(word: &str) -> HashSet<(&str, &str, i8)> {
    let mut pairs: HashSet<(&str, &str, i8)> = HashSet::new();
    for (position, letter) in word.trim().split_whitespace().enumerate(){
        let pair: Vec<&str> = letter.split('-').collect();
        let (letter, status) = (pair[0], pair[1]);

        if is_unique_char(letter, word) || status != "B" {
            pairs.insert((letter, status, position as i8));
        }
    }

    pairs
}

fn tokenize_word(word: &str) -> Vec<Status> {
    let pairs = get_pairs(word);
    let mut tokens: Vec<Status> = vec![];

    for (letter, status, position) in pairs.into_iter() {
        match status {
            "G" => tokens.push(Status::Green(letter, position.try_into().unwrap())),
            "O" => tokens.push(Status::Orange(letter)),
            "B" => tokens.push(Status::Black(letter)),
            token => {
                eprintln!("Unexpected token: {token}");
                std::process::exit(1);
            }
        }
    }
    tokens
}

fn filter_words_with_matches<'a>(tokens: &[Status], wordle_words: &'a mut Vec<&str>) {
    let mut matches: Vec<&str> = wordle_words.clone();

    for token in tokens.iter() {
        matches = match token {
            Status::Green(letter, position) => matches
                .into_iter()
                .filter(|word| {
                    word.contains(letter)
                        && word.chars().nth(*position).unwrap() == letter.chars().next().unwrap()
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

    for _ in 0..5 {
        let word = get_word();
        let tokenized_word = tokenize_word(&word);
        filter_words_with_matches(&tokenized_word, &mut wordle_words);

        println!("---------------");
        for word in wordle_words.iter() {
            println!("{word}");
        }
        println!("---------------\n");
    }
}
