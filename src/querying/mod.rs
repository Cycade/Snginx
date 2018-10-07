mod vector_space_search;
mod bayesian_search;

pub fn vector_space_search(query: &str, index_path: &str, size: i32) -> String {
    vector_space_search::make_query(query, index_path, size)
}

pub fn bayesian_search(query: &str, index_path: &str, doc_num: i32, size: i32) -> String {
    bayesian_search::make_query(query, index_path, doc_num, size)
}