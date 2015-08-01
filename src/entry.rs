use matching;
use std::path::PathBuf;
use std::collections::hash_map::HashMap;

pub struct Entry {
    pub path: PathBuf,
    pub index: HashMap<char, Vec<usize>>,
}

pub fn new(path: String) -> Entry {
    Entry{
        // Build the index before we transfer ownership of path.
        index: matching::index_subject(&path),
        path: PathBuf::from(path),
    }
}
