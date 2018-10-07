use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn load_index(path: &str) -> IndexSet {
    let path = Path::new(path);
    let mut file = File::open(&path).expect("Couldn't found index file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Couldn't read from index file");

    IndexSet::new(input)
}

#[derive(Debug)]
pub struct IndexSet {
    doc_num: i32,
    content: Vec<TermDoc>,
}

impl IndexSet {
    fn new(input: String) -> IndexSet {
        let mut result = IndexSet { doc_num: 0, content: vec![] };

        let raw_list: Vec<&str> = input.split("\r\n").collect();
        let mut count = 0;
        while (count + 3 <= raw_list.len()) {
            let f = String::from(*raw_list.get(count).unwrap());
            let s = String::from(*raw_list.get(count + 1).unwrap());
            let t = String::from(*raw_list.get(count + 2).unwrap());
            let tc = TermDoc::new(f, s, t);
            result.doc_num += 1;
            result.content.push(tc);
            count += 3;
        }

        result
    }

    pub fn search_term(&self, term: String) -> Option<HashMap<String, i32>> {
        let mut result: HashMap<String, i32> = HashMap::new(); 
        for termdoc in &self.content {
            if (termdoc.term == term) {
                result = termdoc.doc.clone();
            }
        }
        match result.len() {
            0 => None,
            _ => Some(result),
        }
    }

    pub fn search_term_idf(&self, term: String) -> Option<f64> {
        for termdoc in &self.content {
            if (termdoc.term == term) {
                return Some(termdoc.tfidf);
            }
        }
        None
    }

    pub fn get_vector(&self, doc: String, terms: Vec<String>) -> Vec<f64> {
        let mut result: Vec<f64> = vec![];
        for term in terms {
            let termIndex = self.search_term(term.clone()).unwrap();
            match termIndex.get(&doc) {
                None => result.push(0.0 as f64),
                Some(&times) => result.push(
                    times as f64 * self.search_term_idf(term).unwrap()
                ),
            };
        }
        result
    }
}

#[derive(Debug)]
struct TermDoc {
    term: String,
    doc: HashMap<String, i32>,
    tfidf: f64,
}

impl TermDoc {
    fn new(f: String, s: String, t: String) -> TermDoc {
        let mut doc_map: HashMap<String, i32> = HashMap::new();
        let raw_list: Vec<&str> = s.split(",").collect();
        let mut count = 0;
        while (count + 2 <= raw_list.len()) {
            let doc = String::from(*raw_list.get(count).unwrap());
            let times = String::from(*raw_list.get(count + 1).unwrap());
            doc_map.insert(doc, times.parse::<i32>().unwrap());
            count += 2;
        }

        TermDoc {
            term: f,
            doc: doc_map,
            tfidf: t.parse::<f64>().unwrap(),
        }
    }
}