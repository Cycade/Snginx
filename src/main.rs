mod tokenizer;
use tokenizer::tokenize;

fn main() {
    let doc2 = "While \ntraversing some 12,000 miles by sea, and 4,000 miles by land.";
    println!("{:?}", tokenize(doc2));


}
