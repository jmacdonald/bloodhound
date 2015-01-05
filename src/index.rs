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

    fn populate(&mut self) {
        let mut directories: fs::Directories;
        let prefix_length = self.path.as_str().unwrap().len()+1;

        // Start by getting any files in the root path.
        match fs::readdir(&self.path) {
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

        // Get an iterator that'll let us walk all of the subdirectories of the index path.
        match fs::walk_dir(&self.path) {
            Ok(iterator) => directories = iterator,
            Err(e) => return,
        }

        for directory in directories {
            // Read the subdirectory entries.
            match fs::readdir(&directory) {
                Ok(entries) => {
                    // Put all of the file-based Path entries into the index.
                    for file in entries.iter().filter(|entry| entry.is_file()) {
                        // Make the file path relative to the index path by stripping it from its string.
                        let file_path = Path::new(file.as_str().unwrap().slice_from(prefix_length));
                        self.entries.push(file_path);
                    }
                },
                Err(e) => continue,
            }
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
