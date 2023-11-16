use std::collections::HashMap;

pub struct Node {
    pub word: String,
    pub children: HashMap<i32, Node>,
}

impl Node {
    pub fn find<'a>(&self, query: &String, threshold: &i32) -> Vec<&'a String> {
        todo!()
    }
}
