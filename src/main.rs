use std::fs;

#[allow(dead_code)]
enum Status {
    Correct(char),
    Close(char),
    NotFound(char),
}

fn get_word_list(file: &String) -> Vec<&str> {
    let mut data = file.split('\n').collect();
    data
}

fn main() {
    let file = match fs::read_to_string("datasets/wordle_word_list.txt") {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    };

    let words = get_word_list(&file);
    let filtered_words: Vec<&&str> = words.iter().filter(|word| word.contains('a') && word.contains('e')).collect();

    for word in filtered_words.iter() {
        println!("{}", word)
    }
    println!("{}", filtered_words.len())
}
