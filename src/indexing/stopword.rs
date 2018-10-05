use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub fn read_stopwords(stopword_path: &str) -> Vec<&'static str> {
    let path = Path::new(stopword_path);
    let mut stopwords: Vec<&'static str> = vec![];
    let mut file = match File::open(path) {
        Err(_) => panic!("Couldn't open {}", path.display()),
        Ok(file) => file,
    };
    
    let mut input =  String::new();
    match file.read_to_string(&mut input) {
        Err(_) => panic!("Couldn't read {}", path.display()),
        Ok(_) => println!("Done!"),
    }

    for word in input.split("\r\n") {
        stopwords.push(Box::leak(word.to_owned().into_boxed_str()));
    }

    stopwords
}