mod stemmer;
mod tokenizer;

use std::collections::HashMap;
type TermIDF = HashMap<String, HashMap<String, i32>>;
// HashMap<Term: String, HashMap<doc_id: String, times: i32>>

pub struct Collection<'a> {
    doc_num: i32,
    term_list: TermIDF,
    stopwords: Vec<&'a str>,
}

impl<'a> Collection<'a> {
    pub fn new(list: Vec<&'a str>) -> Collection<'a> {
        Collection { doc_num: 0, term_list: HashMap::new(), stopwords: list }
    }

    pub fn insert_doc(&mut self, doc_id: &str, content: &str) {
        let doc = IndexedDoc::new(doc_id, content);
        for t in doc.indexing_map.keys() {
            if self.stopwords.contains(&t.as_ref()) {
                continue;
            }
            self.term_list.entry(t.to_string()).or_insert(HashMap::new()).insert(doc.doc_id.to_string(), doc.indexing_map[t]);
        }
        self.doc_num += 1;
    }

    pub fn display(&self) -> String {
        let mut result = String::from("");
        for t in self.term_list.keys() {
            result.push_str(&t);
            result.push('\n');
            for (doc, fre) in &self.term_list[t] {
                result.push_str(doc);
                result.push(',');
                result.push_str(&fre.to_string());
                result.push(',');
            }
            let idf = (self.doc_num as f64 / self.term_list[t].len() as f64).log2();

            result.push_str(&idf.to_string());
            result.push('\n');
        }
        result
    }
}

struct IndexedDoc<'a> {
    doc_id: &'a str,
    indexing_map: HashMap<String, i32>,
}

impl<'a> IndexedDoc<'a> {
    fn new(id: &'a str, content: &'a str) -> IndexedDoc<'a> {
        IndexedDoc { doc_id: id, indexing_map: indexing(content) }
    }
}

fn indexing(doc: &str) -> HashMap<String, i32> {
    let raw_map = tokenizer::tokenize(doc);
    let mut stemmed_map: HashMap<String, i32> = HashMap::new();
    for (term, fre) in &raw_map {
        let stemmed_term = stemmer::get(term).unwrap();
        let count = stemmed_map.entry(stemmed_term).or_insert(0);
        *count += fre;
    }
    stemmed_map
}