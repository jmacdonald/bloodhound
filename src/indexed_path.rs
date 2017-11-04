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

impl<'a> AsStr for &'a IndexedPath {
    fn as_str(&self) -> &str {
        &self.path_string
    }
}

impl IndexedPath {
    pub fn new(path: PathBuf) -> IndexedPath {
        let path_string = path.to_string_lossy().into_owned();

        IndexedPath{
            path: path,
            path_string: path_string
        }
    }
}