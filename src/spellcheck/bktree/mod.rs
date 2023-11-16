use std::collections::HashMap;
use std::fs;

mod node;
use crate::spellcheck::bloomfilter::BloomFilter;

pub struct BkTreeChecker {
    root: node::Node,
    bloom_filter: BloomFilter,
}

fn read_dictionary() -> Vec<String> {
    let contents =
        fs::read_to_string("/tmp/dictionary").expect("Should have been able to read the file");
    return contents.lines().map(|line| line.to_string()).collect();
}

impl BkTreeChecker {
    pub fn new() -> BkTreeChecker {
        let mut checker = BkTreeChecker {
            root: node::Node {
                word: "foobar".to_string(),
                children: HashMap::new(),
            },
            bloom_filter: BloomFilter::new(7000000, 16),
        };
        let dictionary = read_dictionary();
        for word in dictionary {
            checker.bloom_filter.insert(&word);
        }
        checker
    }

    pub fn check(&self, word: &String) {
        if self.bloom_filter.not_exist(word) {
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
