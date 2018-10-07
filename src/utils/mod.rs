mod stemmer;
mod tokenizer;
pub mod index_loader;

use std::collections::HashMap;
use self::index_loader::IndexSet;

pub fn doc_index(doc: &str) -> HashMap<String, i32> {
    let raw_map = tokenizer::tokenize(doc);
    let mut stemmed_map: HashMap<String, i32> = HashMap::new();
    for (term, fre) in &raw_map {
        let stemmed_term = stemmer::get(term).unwrap();
        let count = stemmed_map.entry(stemmed_term).or_insert(0);
        *count += fre;
    }
    stemmed_map
}

pub fn load_index(path: &str) -> IndexSet {
    index_loader::load_index(path)
}