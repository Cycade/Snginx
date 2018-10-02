mod indexing;
use indexing::Collection;

fn main() {
    let list = vec!["a", "some"];
    let mut c = Collection::new(list);
    let doc1 = "Melbourne is a city not far form a sea, some sea, I mean";
    let doc2 = "While \ntraversing some 12,000 miles by sea, and 4,000 miles by land.";

    c.insert_doc("doc1", doc1);
    c.insert_doc("doc2", doc2);
    println!("{}", c.display());
}
