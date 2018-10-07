use std::collections::HashMap;
use std::cmp::Ordering;

use utils::load_index;
use utils::doc_index;
use utils::index_loader::IndexSet;

pub fn make_query(query: &str, index_path: &str) -> String {
    let mut r = ResultSet::new(query, index_path);
    r.find_possible();
    r.display()
}

struct ResultSet {
    term_vec: Vec<String>,
    query: DocVec,
    alter_set: Vec<DocVec>,
    collection: IndexSet,
}

impl ResultSet {
    fn new(q: &str, path: &str) -> ResultSet {
        let c = load_index(path);
        let raw_query = doc_index(q);

        let mut idf_query: Vec<f64> = vec![];
        let mut term_vec: Vec<String> = vec![];

        for (term, fre) in raw_query {
            match c.search_term_idf(term.clone()) {
                None => continue,
                _ => ()
            };
            term_vec.push(term.clone());
            idf_query.push(fre as f64 * c.search_term_idf(term).unwrap());
        }

        let this_query = DocVec {
            doc_id: "Query".to_string(),
            vector: idf_query,
            similarity: 0.0, 
        };

        ResultSet { term_vec: term_vec, query: this_query, alter_set: vec![], collection: c }
    }

    fn find_possible(&mut self) {
        let mut possible_set: Vec<String> = vec![];
        let search_set = self.term_vec.clone();
        for term in &self.term_vec {
            let search_result = &self.collection.search_term(term.clone()).unwrap();
            for doc in search_result.keys() {
                if possible_set.contains(doc) {
                    continue;
                }
                possible_set.push(doc.to_string());

                let term_vec = self.collection.get_vector(doc.to_string(), search_set.clone());
                let mut doc_vec = DocVec { doc_id: doc.to_string(), vector: term_vec, similarity: 0.0 };
                doc_vec.cos_sim(&self.query);
                self.alter_set.push(doc_vec);
            }
        }
        self.alter_set.sort_by(|a, b| {
            if a.similarity > b.similarity {
                return Ordering::Less;
            } else if a.similarity < b.similarity {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        });
    }

    fn display(&self) -> String {
        let mut result = String::new();

        result.push_str("Term to search: ");
        result.push_str(&self.term_vec.join(" "));
        result.push_str("\r\n");

        result.push_str(&self.query.display().as_str());
        result.push_str("\r\n");

        for doc in &self.alter_set {
            result.push_str(doc.display().as_str());
            result.push_str("\r\n");

        }

        result
    }
}

struct DocVec {
    doc_id: String,
    vector: Vec<f64>,
    similarity: f64,
}

impl DocVec {
    fn cos_sim(&mut self, query: &DocVec) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 0..query.vector.len() {
            sum += (self.vector[i] - query.vector[i]).powf(2.0);
        }
        self.similarity = sum.sqrt();
        sum.sqrt()
    }

    fn display(&self) -> String {
        format!("{}: {} with {:?}", self.doc_id, self.similarity, self.vector)
    }
}