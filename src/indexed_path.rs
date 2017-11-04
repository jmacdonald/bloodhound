use fragment::matching::AsStr;
use std::path::PathBuf;

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
