mod indexing;
mod querying;

use indexing::Collection;
use std::path::Path;
use querying::loader::load_index;

fn main() {
    // indexing::make_index("resources", "stopwords.txt");
    
    let f = load_index("indexing.txt");
    println!("{:?}", f);
}
