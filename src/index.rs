struct Index {
    path: String,
    entries: Vec<String>,
}

impl Index {
    fn new(path: &str) -> Index {
        return Index { path: path.to_string(), entries: vec![] }
    }
}

#[test]
fn new_creates_index_with_passed_path() {
    let path = "my path";
    let index = Index::new(path);
    assert!(index.path == path);
}

#[test]
fn new_creates_index_with_empty_array() {
    let path = "my path";
    let index = Index::new(path);
    let empty_array: Vec<String> = vec![];
    assert!(index.entries == empty_array)
}
