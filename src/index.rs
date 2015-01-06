use std::io::fs;
use std::io::fs::PathExtensions;
use std::str::StrExt;

struct Index {
    path: Path,
    entries: Vec<Path>,
}

impl Index {
    fn new(path: &str) -> Index {
        return Index { path: Path::new(path.to_string()), entries: vec![] }
    }

    // Finds all files inside and beneath the index path
    // and adds them to the index entries vector.
    fn populate(&mut self) {
        // The entries listed by readdir include the root index path; we want
        // relative paths, so we get this length so that we can strip it.
        let prefix_length = self.path.as_str().unwrap().len()+1;

        // Start by getting any files in the root path. Since
        // read_directory_entries is already borrowing a mutable
        // reference to the index, so we can't also lend out a
        // reference to its path field, hence the clone.
        let path = self.path.clone();
        self.index_directory_files(path, prefix_length);

        // Get an iterator that'll let us walk all of the subdirectories
        // of the index path, bailing out if there's an error of any kind.
        let mut subdirectories = match fs::walk_dir(&self.path) {
            Ok(iterator) => iterator,
            Err(e) => return,
        };

        // Index any other files beneath the root directory.
        for directory in subdirectories {
            self.index_directory_files(directory, prefix_length);
        }
    }

    // Helper method for populate.
    // Finds files for a particular directory, strips prefix_length leading
    // characters from their path, and adds them to the index entries vector.
    fn index_directory_files(&mut self, directory: Path, prefix_length: uint) {
        match fs::readdir(&directory) {
            Ok(entries) => {
                // Put all of the file-based Path entries into the index.
                for file in entries.iter().filter(|entry| entry.is_file()) {
                    // Make the file path relative to the index path by stripping it from its string.
                    let file_path = Path::new(file.as_str().unwrap().slice_from(prefix_length));

                    self.entries.push(file_path);
                }
            },
            Err(e) => (),
        }
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

    assert!(index.entries == empty_array);
}

#[test]
fn populate_adds_all_files_to_entries() {
    let path = "tests/sample";
    let index = &mut Index::new(path);
    let expected_array = vec![Path::new("root_file"), Path::new("directory/nested_file")];
    index.populate();

    assert!(index.entries == expected_array);
}
