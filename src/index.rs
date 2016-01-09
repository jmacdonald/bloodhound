extern crate fragment;

use self::fragment::matching;
use walkdir::{DirEntry, Error, WalkDir};
use std::path::PathBuf;
use std::clone::Clone;

pub struct Index {
    path: PathBuf,
    entries: Vec<IndexedPath>,
}

#[derive(Debug, PartialEq)]
pub struct IndexedPath(PathBuf);

impl ToString for IndexedPath {
    fn to_string(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Clone for IndexedPath {
    fn clone(&self) -> Self {
        IndexedPath(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone()
    }
}

impl Index {
    pub fn new(path: PathBuf) -> Index {
        Index {
            path: path,
            entries: Vec::new(),
        }
    }

    /// Finds all files inside and beneath the index path
    /// and adds them to the index entries vector.
    pub fn populate(&mut self) {
        // The entries listed by read_dir include the root index path; we want
        // relative paths, so we get this length so that we can strip it.
        let prefix_length = match self.path.to_str() {
            Some(path) => path.len() + 1,
            None => return,
        };

        // Start indexing at the specified path.
        for entry in WalkDir::new(&self.path) {
            match relative_entry_path(entry, prefix_length) {
                Some(entry_path) => self.entries.push(IndexedPath(PathBuf::from(entry_path))),
                _ => (),
            }
        }
    }

    pub fn find(&self, term: &str, limit: usize) -> Vec<PathBuf> {
        matching::find(term, &self.entries, limit)
            .into_iter()
            .map(|r| {
                let IndexedPath(other_thing) = r.clone();
                other_thing
            })
            .collect()
    }
}

/// Transforms a DirEntry object into an optional relative path string,
/// returning None if any errors occur or if the entry is not a file.
fn relative_entry_path(entry: Result<DirEntry, Error>, prefix_length: usize) -> Option<String> {
    match entry {
        Ok(e) => {
            match e.path().metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        match e.path().to_str() {
                            Some(path) => Some(path[prefix_length..].to_string()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    extern crate fragment;

    use super::{Index, IndexedPath};
    use std::path::PathBuf;
    use self::fragment::matching;

    #[test]
    fn populate_adds_all_files_to_entries() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![IndexedPath(PathBuf::from("directory/nested_file")),
                                    IndexedPath(PathBuf::from("root_file"))];
        index.populate();

        assert_eq!(index.entries, expected_entries);
    }

    #[test]
    fn find_defers_to_matching_module() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        index.populate();
        let term = "root";
        let limit = 5;

        // Get a string version of the results (PathBuf doesn't implement the display trait).
        let results: Vec<String> = index.find(term, limit)
                                        .iter()
                                        .map(|r| r.to_string_lossy().into_owned())
                                        .collect();

        assert_eq!(results,
                   vec!["root_file".to_string(), "directory/nested_file".to_string()]);
    }
}
