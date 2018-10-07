mod indexing;
mod querying;
mod utils;

use indexing::Collection;
use std::path::Path;

fn main() {
    // indexing::make_index("resources", "stopwords.txt");
    
    let result = querying::vector_space_search(
        "are experimental pressure distributions on bodies of revolution at angle of attack available",
        "indexing.txt"
    );
    println!("{}", result);
    // let f = load_index("indexing.txt");
    // println!("{:?}", f);
}
