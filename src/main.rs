mod indexing;
mod querying;
mod utils;

use indexing::Collection;
use std::path::Path;

fn main() {
    // indexing::make_index("resources", "stopwords.txt");
    
    let query = "are experimental pressure distributions on bodies of revolution at angle of attack available";
    let result = querying::vector_space_search(
        query,
        "indexing.txt",
        30
    );
    println!("{}", result);

    let result2 = querying::bayesian_search(
        query,
        "indexing.txt",
        1400,
        30
    );
    println!("{}", result2);
    // let f = load_index("indexing.txt");
    // println!("{:?}", f);
}
