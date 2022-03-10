use std::fs;
use std::io;

#[derive(Debug)]
enum LetterStatus<'a> {
    Correct(&'a str, usize),
    Close(&'a str),
    NotFound(&'a str),
}

fn get_word_list(file: &String) -> Vec<&str> {
    let data = file.split('\n').collect();
    data
}

// TODO(Hícaro): Big and unclear, variable names aren't that good
fn tokenize_word(word: &str) -> Vec<LetterStatus> {
    let letters: Vec<&str> = word.split_whitespace().collect();

    let pairs: Vec<(&str, &str)> = letters.into_iter().map(|pair| {
        let current_pair: Vec<&str> = pair.split('-').collect();
        (current_pair[0], current_pair[1])
    }).collect();

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

fn main() {
    let file = match fs::read_to_string("datasets/wordle_word_list.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
    let wordle_words = get_word_list(&file);

    let mut i = 5;
    while i != 0 {
        let word = get_word();
        let tokenized_word = tokenize_word(&word);
        i-=1;
    }
}
