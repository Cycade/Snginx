use std::collections::HashMap;
use std::cmp::Ordering;

use utils::load_index;
use utils::doc_index;
use utils::index_loader::IndexSet;

pub fn make_query(query: &str, index_path: &str, doc_num: i32, size: i32) -> String {
    let mut r = ResultSet::new(query, index_path, doc_num);
    r.find_possible();
    r.display(size)
}

struct ResultSet {
    term_df: HashMap<String, f64>,
    alter_set: Vec<(String, f64)>,
    collection: IndexSet,
}

impl ResultSet {
    fn new(q: &str, path: &str, doc_num: i32) -> ResultSet {
        let c = load_index(path);
        let raw_query = doc_index(q);

        let mut term_df: HashMap<String, f64> = HashMap::new();

        for term in raw_query.keys() {
            if c.search_term_idf(term.clone()) == None {
                continue;
            }
            let rank_ratio = ((doc_num as f64 + 0.5) / (c.search_term(term.clone()).unwrap().len() as f64 + 0.5)).ln();
            term_df.insert(term.clone(), rank_ratio);
        }

        ResultSet { term_df: term_df, alter_set: vec![], collection: c }
    }

    fn find_possible(&mut self) {
        let mut possible_set: Vec<String> = vec![];
        for term in self.term_df.keys() {
            let search_result = self.collection.search_term(term.clone()).unwrap();
            for doc in search_result.keys() {
                if possible_set.contains(doc) {
                    continue;
                }
                possible_set.push(doc.to_string());
                let mut w: f64 = 0.0;
                for docterm in self.term_df.keys() {
                    if self.collection.search_term(docterm.clone()).unwrap().get(doc) != None {
                        w += self.term_df[docterm];
                    }
                }
                self.alter_set.push((doc.to_string(), w));
            }
        }
        self.alter_set.sort_by(|a, b| {
            if a.1 > b.1 {
                return Ordering::Less;
            } else if a.1 < b.1 {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        });
    }

    fn display(&self, size: i32) -> String {
        let mut result = String::new();

        for (term, rank) in &self.term_df {
            result.push_str(&term);
            result.push_str(": ");
            result.push_str(rank.to_string().as_str());
            result.push_str(", ");
        }
        result.push_str("\r\n");

        let mut count = 0;
        for (doc, w) in &self.alter_set {
            if count == size {
                break;
            }
            result.push_str(&doc);
            result.push_str(": ");
            result.push_str(w.to_string().as_str());
            result.push_str("\r\n");
            count += 1;
        }
        result
    }
}