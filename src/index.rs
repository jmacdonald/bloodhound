use std::fs;
use std::fs::PathExt;
use std::path::{Path, PathBuf};

struct Index {
    path: PathBuf,
    entries: Vec<PathBuf>,
}

impl Index {
    fn new(path: &str) -> Index {
        return Index { path: PathBuf::from(path.to_string()), entries: vec![] }
    }

    // Finds all files inside and beneath the index path
    // and adds them to the index entries vector.
    fn populate(&mut self) {
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

    // Helper method for populate.
    // Finds files for a particular directory, strips prefix_length leading
    // characters from their path, and adds them to the index entries vector.
    fn index_directory_files(&mut self, directory: &Path, prefix_length: usize) {
        match fs::read_dir(directory) {
            Ok(entries) => {
                // Put all of the file-based Path entries into the index.
                for entry in entries {
                    match entry {
                        Ok(e) => {
                            match e.path().metadata() {
                                Ok(metadata) => if metadata.is_file() {
                                    match e.path().to_str() {
                                        Some(entry_path) => {
                                            // Make the file path relative to the index
                                            // path by stripping it from its string.
                                            let relative_path = entry_path[prefix_length..].to_string();

                                            self.entries.push(PathBuf::from(relative_path));
                                        },
                                        None => (),
                                    }
                                },
                                Err(_) => (),
                            }
                        },
                        Err(_) => (),
                    }
                }
            },
            Err(_) => (),
        }
    }
}

#[test]
fn new_creates_index_with_passed_path() {
    let path = "my path";
    let index = Index::new(path);

    // Get the index path as a string.
    let index_path = match index.path.to_str() {
        Some(value) => value,
        None => ""
    };

    assert_eq!(index_path, path);
}

#[test]
fn new_creates_index_with_empty_array() {
    let path = "my path";
    let index = Index::new(path);
    let empty_array: Vec<PathBuf> = vec![];

    assert!(index.entries == empty_array);
}

#[test]
fn populate_adds_all_files_to_entries() {
    let path = "tests/sample";
    let index = &mut Index::new(path);
    let expected_array = vec![PathBuf::from("root_file"), PathBuf::from("directory/nested_file")];
    index.populate();

    assert!(index.entries == expected_array);
}
