mod vector_space_search;

pub fn vector_space_search(query: &str, index_path: &str) -> String {
    vector_space_search::make_query(query, index_path)
}