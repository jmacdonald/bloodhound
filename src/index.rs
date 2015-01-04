struct Index {
    path: Path,
    entries: Vec<Path>,
}

impl Index {
    fn new(path: &str) -> Index {
        return Index { path: Path::new(path.to_string()), entries: vec![] }
    }
}

#[test]
fn new_creates_index_with_passed_path() {
    let path = "my path";
    let index = Index::new(path);

    // Get the index path as a string.
    let index_path = match index.path.as_str() {
        Some(value) => value,
        None => ""
    };

    assert_eq!(index_path, path);
}

#[test]
fn new_creates_index_with_empty_array() {
    let path = "my path";
    let index = Index::new(path);
    let empty_array: Vec<Path> = vec![];

    assert!(index.entries == empty_array)
}
