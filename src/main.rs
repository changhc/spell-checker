use std::fs;
mod spellcheck;
use spellcheck::bktree;

fn main() {
    let file_path = std::env::args().nth(1).expect("filepath is missing");
    let checker = bktree::BkTreeChecker::new();
    read_file(file_path)
        .iter()
        .for_each(|word| checker.check(word));
}

fn read_file(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents
        .split_whitespace()
        .map(|x| {
            x.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
        })
        .collect();
}
