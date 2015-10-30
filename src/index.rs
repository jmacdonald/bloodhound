use matching;

use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::io::Error;

pub struct Index {
    path: PathBuf,
    entries: Vec<PathBuf>,
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
            Some(path) => path.len()+1,
            None => return,
        };

        // Start by getting any files in the root path. Since
        // read_directory_entries is already borrowing a mutable
        // reference to the index, so we can't also lend out a
        // reference to its path field, hence the clone.
        let path = self.path.clone();
        self.index_directory_files(&path, prefix_length);

        // Get an iterator that'll let us walk all of the subdirectories
        // of the index path, bailing out if there's an error of any kind.
        match fs::walk_dir(&self.path) {
            Ok(subdirectories) => {
                // Index any other files beneath the root directory.
                for directory in subdirectories {
                    match directory {
                        Ok(dir) => self.index_directory_files(&dir.path(), prefix_length),
                        Err(_) => (),
                    }
                }
            },
            Err(_) => return,
        };
    }


    pub fn find(&self, term: &str, limit: usize) -> Vec<matching::Result> {
        matching::find(term, &self.entries, limit)
    }

    /// Helper method for populate.
    /// Finds files for a particular directory, strips prefix_length leading
    /// characters from their path, and adds them to the index entries vector.
    fn index_directory_files(&mut self, directory: &Path, prefix_length: usize) {
        match fs::read_dir(directory) {
            Ok(entries) => {
                // Put all of the file-based Path entries into the index.
                for entry in entries {
                    match relative_entry_path(entry, prefix_length) {
                        Some(entry_path) => self.entries.push(PathBuf::from(entry_path)),
                        _ => (),
                    }
                }
            },
            Err(_) => (),
        }
    }
}

/// Transforms a DirEntry object into an optional relative path string,
/// returning None if any errors occur or if the entry is not a file.
fn relative_entry_path(entry: Result<DirEntry, Error>, prefix_length: usize) -> Option<String> {
    match entry {
        Ok(e) => match e.path().metadata() {
            Ok(metadata) => if metadata.is_file() {
                match e.path().to_str() {
                    Some(path) => Some(path[prefix_length..].to_string()),
                    _ => None,
                }
            } else {
                None
            },
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::Index;
    use std::path::PathBuf;

    #[test]
    fn populate_adds_all_files_to_entries() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![
            PathBuf::from("root_file".to_string()),
            PathBuf::from("directory/nested_file".to_string())
        ];
        index.populate();

        assert!(index.entries == expected_entries);
    }

    #[test]
    fn find_defers_to_matching_module() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        index.populate();
        let term = "root";
        let limit = 5;
        let expected_results = ::matching::find(
            term,
            &vec![
                PathBuf::from("root_file".to_string()),
                PathBuf::from("directory/nested_file".to_string())
            ],
            limit
        );

        assert_eq!(index.find(term, limit), expected_results);
    }
}
