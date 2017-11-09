use fragment::matching::AsStr;
use std::path::PathBuf;

/// This Path + String pair exists so that we can build the path's string
/// representation once when populating the index, rather than on each search.
/// It's also how we support case insensitive indexing, where we need to store
/// a lowercased search value without modifying the case-sensitive PathBuf.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IndexedPath {
    pub path: PathBuf,
    pub path_string: String,
}

impl AsStr for IndexedPath {
    fn as_str(&self) -> &str {
        &self.path_string
    }
}

impl IndexedPath {
    pub fn new(path: &str, case_sensitive: bool) -> IndexedPath {
        IndexedPath{
            path: PathBuf::from(path),
            path_string: path.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use fragment::matching::AsStr;
    use std::path::PathBuf;
    use super::IndexedPath;

    #[test]
    fn new_lowercases_str_representation_when_case_sensitive_is_false() {
        let indexed_path = IndexedPath::new("Cargo.toml", false);
        assert_eq!(indexed_path.as_str(), "cargo.toml");
    }

    #[test]
    fn new_leaves_str_representation_as_is_when_case_sensitive_is_true() {
        let indexed_path = IndexedPath::new("Cargo.toml", true);
        assert_eq!(indexed_path.as_str(), "Cargo.toml");
    }

    #[test]
    fn new_leaves_path_as_is_regardless_of_case_sensitivity() {
        let indexed_path = IndexedPath::new("Cargo.toml", false);
        let case_sensitive_indexed_path = IndexedPath::new("Cargo.toml", true);

        assert_eq!(indexed_path.path, PathBuf::from("Cargo.toml"));
        assert_eq!(case_sensitive_indexed_path.path, PathBuf::from("Cargo.toml"));
    }
}
