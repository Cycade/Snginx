extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
use self::regex::Captures;

#[derive(Debug)]
struct Doc {
    content: String,
    index_map: HashMap<String, i32>,
}

impl<'a> Doc {
    fn new(input: &'a str) -> Doc {
        Doc { content: input.to_string(), index_map: HashMap::new() }
    }

    fn retrive_and_replace(&mut self, pat: Regex) {
        for cap in pat.captures_iter(&self.content) {
            let token = cap.get(0).unwrap().as_str().to_string();
            let count = self.index_map.entry(token).or_insert(0);
            *count += 1;
        }
        self.content = pat.replace_all(&self.content, "").to_string();
    }

    fn remove_hyphenation(&mut self) {
        let pat = Regex::new(r"(?P<pre>\w+)-\n(?P<post>\w+)").unwrap();
        self.content = pat.replace_all(&self.content, |caps: &Captures| {
            format!("\n{}{}", &caps.name("pre").unwrap().as_str(),
                              &caps.name("post").unwrap().as_str())
        }).to_string();
    }

    fn retrive_email(&mut self) {
        let pat = Regex::new(r"[\w.]+@\w+.[a-zA-Z]+").unwrap();
        self.retrive_and_replace(pat);
    }

    fn retrive_http(&mut self) {
        let pat = Regex::new(r"(http(s)*://)*[a-zA-Z0-9]+(\.[a-zA-Z0-9]+)+(/[a-zA-Z0-9$-_.+!*'(),%]+\w)*(/)?").unwrap();
        self.retrive_and_replace(pat);
    }

    fn retrive_ipaddress(&mut self) {
        let pat = Regex::new(r"((2[0-4]\d|25[0-5]|[01]?\d\d?)\.){3}(2[0-4]\d|25[0-5]|[01]?\d\d?)").unwrap();
        self.retrive_and_replace(pat);
    }

    fn retrive_quoted(&mut self) {
        let pat = Regex::new(r#"((\n| )"[^"]+")|((\n| )'[^']+')"#).unwrap();
        self.retrive_and_replace(pat);
    }

    fn retrive_caps(&mut self) {
        let pat = Regex::new(r"[A-Z][^ ]*\w(\s[A-Z][^ ]*\w)+").unwrap();
        self.retrive_and_replace(pat);
    }

    fn retrive_acronym(&mut self) {
        let pat = Regex::new(r"[A-Z](\.?[A-Z])+\.?").unwrap();
        for cap in pat.captures_iter(&self.content) {
            let token = cap.get(0).unwrap().as_str().to_string().replace(".", "");
            let count = self.index_map.entry(token).or_insert(0);
            *count += 1;
        }
        self.content = pat.replace_all(&self.content, "").to_string();
    }

    fn split(&mut self) {
        let mut temp = String::from("");
        for ch in self.content.chars() {
            if ch.is_whitespace() && temp.len() != 0 {
                let count = self.index_map.entry(temp).or_insert(0);
                *count += 1;
                temp = String::from("");
            } else if ch.is_alphabetic() || ch.is_ascii_digit() {
                if ch.is_ascii_uppercase() {
                    temp.push(ch.to_ascii_lowercase());                    
                } else {
                    temp.push(ch);
                }
            }
        }
        if (temp != "") {
            let count = self.index_map.entry(temp).or_insert(0);
            *count += 1;
        }

    }
}

pub fn tokenize(input: &str) -> HashMap<String, i32> {
    let mut doc = Doc::new(input);
    doc.remove_hyphenation();
    doc.retrive_email();
    doc.retrive_http();
    doc.retrive_ipaddress();
    doc.retrive_quoted();
    doc.retrive_caps();
    doc.retrive_acronym();
    doc.split();
    doc.index_map
}


#[test]
fn hyphen_test() {
    let mut doc1 = Doc::new("He removed some infor-\nmation from the paper. After that he left for coloni-\nzation.");
    doc1.remove_hyphenation();
    assert_eq!(doc1.content, String::from("He removed some \ninformation from the paper. After that he left for \ncolonization."));
}

#[test]
fn email_test() {
    let mut doc1 = Doc::new("example@163.com, guest@gmail.com, guest@gmail.com, Chris.messom@monash.edu");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("example@163.com".to_string(), 1);
    doc1_map.insert("guest@gmail.com".to_string(), 2);
    doc1_map.insert("Chris.messom@monash.edu".to_string(), 1);
    doc1.retrive_email();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn http_test() {
    let mut doc1 = Doc::new("https://rustcc.gitbooks.io/1.23.4/content/editors/vscode.html, \
    http://www.civclub.net/html_c4/civdoc/civ4quote.htm, www.google.au, www.google.au");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("https://rustcc.gitbooks.io/1.23.4/content/editors/vscode.html".to_string(), 1);
    doc1_map.insert("www.google.au".to_string(), 2);
    doc1_map.insert("http://www.civclub.net/html_c4/civdoc/civ4quote.htm".to_string(), 1);
    doc1.retrive_http();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn ipaddress_test() {
    let mut doc1 = Doc::new("283.400.2.4, 40.30.29.208, 94.04.3");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("40.30.29.208".to_string(), 1);
    doc1.retrive_ipaddress();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn quoted_test() {
    let mut doc1 = Doc::new("\"forum\" is not 'efficient', 'efficient', in \"Faculty of IT\", you know?");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("\"forum\"".to_string(), 1);
    doc1_map.insert("'efficient'".to_string(), 2);
    doc1_map.insert("\"Faculty of IT\"".to_string(), 1);
    doc1.retrive_quoted();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn caps_test() {
    let mut doc1 = Doc::new("Find a Date and Faculty of IT, write HOLY CODE.");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("HOLY CODE".to_string(), 1);
    doc1.retrive_caps();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn acronym_test() {
    let mut doc1 = Doc::new("C.A.T U.S.A CA USA");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("CAT".to_string(), 1);
    doc1_map.insert("USA".to_string(), 2);
    doc1_map.insert("CA".to_string(), 1);
    doc1.retrive_acronym();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn split_test() {
    let mut doc1 = Doc::new("To be or not to be, not or to be.");
    let mut doc1_map = HashMap::new();
    doc1_map.insert("to".to_string(), 3);
    doc1_map.insert("be".to_string(), 3);
    doc1_map.insert("or".to_string(), 2);
    doc1_map.insert("not".to_string(), 2);
    doc1.split();
    assert_eq!(doc1.index_map, doc1_map);
}

#[test]
fn tokenize_test() {
    let doc1 = "To be or not to be, not or to be.";
    let mut doc1_map = HashMap::new();
    doc1_map.insert("to".to_string(), 3);
    doc1_map.insert("be".to_string(), 3);
    doc1_map.insert("or".to_string(), 2);
    doc1_map.insert("not".to_string(), 2);
    assert_eq!(tokenize(doc1), doc1_map);

    let doc2 = "While \ntraversing some 12,000 miles by sea, and 4,000 miles by land.";
    let doc2_map = tokenize(doc2);
    assert_eq!(doc2_map.contains_key("while"), true);
    assert_eq!(doc2_map.contains_key("4000"), true);
}