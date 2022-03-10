use std::fs;
use std::io;

#[derive(Debug)]
enum LetterStatus<'a> {
    Correct(&'a str, usize),
    Close(&'a str),
    NotFound(&'a str),
}

// TODO(Hícaro): Big and unclear, variable names aren't that good
fn tokenize_word(word: &str) -> Vec<LetterStatus> {
    let letters: Vec<&str> = word.split_whitespace().collect();

    let pairs: Vec<(&str, &str)> = letters
        .into_iter()
        .map(|pair| {
            let current_pair: Vec<&str> = pair.split('-').collect();
            (current_pair[0], current_pair[1])
        })
        .collect();

    let mut tokens: Vec<LetterStatus> = vec![];
    for (i, pair) in pairs.into_iter().enumerate() {
        let (letter, status) = pair;

        match status {
            "C" => tokens.push(LetterStatus::Correct(letter, i)),
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

fn get_word() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected a string");
    input
}

// TODO(Hícaro): Eliminate code repetition
fn get_matches<'a>(tokens: &'a Vec<LetterStatus>, wordle_words: &'a Vec<&str>) -> Vec<&'a &'a str> {
    let mut filtered_wordle_words: Vec<&&str> = vec![];

    for token in tokens.into_iter() {
        let mut matches: Vec<&&str> = match token {
            LetterStatus::Correct(letter, position) => wordle_words
                .iter()
                .filter(|word| {
                    word.contains(letter)
                        && word.chars().nth(*position).unwrap() == letter.chars().nth(0).unwrap()
                })
                .collect(),

            LetterStatus::Close(letter) => wordle_words
                .iter()
                .filter(|word| word.contains(letter))
                .collect(),

            LetterStatus::NotFound(letter) => wordle_words
                .iter()
                .filter(|word| !word.contains(letter))
                .collect(),
        };

        filtered_wordle_words.append(&mut matches)
    }

    filtered_wordle_words
}

fn main() {
    let file = match fs::read_to_string("datasets/wordle_word_list.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let wordle_words = file.split('\n').collect();

    let mut i = 5;
    while i != 0 {
        let word = get_word();
        let tokenized_word = tokenize_word(&word);
        let filtered_words = get_matches(&tokenized_word, &wordle_words);

        for word in filtered_words.into_iter() {
            println!("{word}")
        }
        i -= 1;
    }
}
