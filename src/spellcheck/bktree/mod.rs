use std::collections::HashMap;
use std::fs;

mod node;
use crate::spellcheck::bloomfilter::BloomFilter;

pub struct BkTreeChecker {
    root: node::Node,
    bloom_filter: BloomFilter,
}

fn capitalise(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn read_dictionary() -> Vec<String> {
    let contents =
        fs::read_to_string("/tmp/words.txt").expect("Cannot read the dictionary");
    return contents
        .lines()
        .map(|line| {
            line.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
        })
        .collect();
}

impl BkTreeChecker {
    pub fn new() -> BkTreeChecker {
        let mut checker = BkTreeChecker {
            root: node::Node {
                word: "foobar".to_string(),
                children: HashMap::new(),
            },
            bloom_filter: BloomFilter::new(27_000_000, 23),
        };
        let dictionary = read_dictionary();
        for word in dictionary {
            checker.bloom_filter.insert(&word);
            checker.bloom_filter.insert(&capitalise(&word));
        }
        checker
    }

    pub fn check(&self, word: &String) {
        if !self.bloom_filter.contains(word) {
            println!("'{}' not found", word);
            return;
        }
        println!("{}", word);
        //let options = self.root.find(word, &3);
        //if options.len() > 0 {
        //    println!("Do you mean: {:?}", options);
        //}
    }
}
