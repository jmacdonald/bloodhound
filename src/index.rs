use fragment::matching;
use ExclusionPattern;
use walkdir::{DirEntry, Error, WalkDir};
use std::path::PathBuf;
use IndexedPath;

pub struct Index {
    path: PathBuf,
    entries: Vec<IndexedPath>,
}

impl Index {
    pub fn new(path: PathBuf) -> Index {
        Index {
            path: path,
            entries: Vec::new()
        }
    }

    /// Finds all files inside and beneath the index path
    /// and adds them to the index entries vector.
    pub fn populate(&mut self, exclusions: Option<Vec<ExclusionPattern>>, case_sensitive: bool) {
        // The entries listed by read_dir include the root index path; we want
        // relative paths, so we get this length so that we can strip it.
        let prefix_length = match self.path.to_str() {
            Some(path) => path.len() + 1,
            None => return,
        };

        // Start indexing at the specified path.
        let filtered_entries = WalkDir::new(&self.path).into_iter().filter_entry(|entry| {
            if let Some(ref exclusions) = exclusions {
                !exclusions.iter().any(|exclusion| {
                    exclusion.matches(entry.path().to_string_lossy().as_ref())
                })
            } else {
                true
            }
        });

        for entry in filtered_entries {
            relative_entry_path(entry, prefix_length).map(|entry_path| {
                self.entries.push(
                    IndexedPath::new(&entry_path, case_sensitive)
                );
            });
        }
    }

    pub fn find(&self, term: &str, limit: usize) -> Vec<&PathBuf> {
        matching::find(term, &self.entries, limit)
            .into_iter()
            .map(|result| &result.path)
            .collect()
    }
}

/// Transforms a DirEntry object into an optional relative path string,
/// returning None if any errors occur or if the entry is not a file.
fn relative_entry_path(entry: Result<DirEntry, Error>, prefix_length: usize) -> Option<String> {
    entry.ok().and_then(|e| {
        // Limit path look-ups to files.
        e.path().metadata().ok().and_then(|metadata| {
            if metadata.is_file() {
                // Map the absolute path to a relative one.
                e.path().to_str().map(|path| path[prefix_length..].to_string())
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::{Index, IndexedPath, ExclusionPattern};
    use std::path::PathBuf;

    #[test]
    fn populate_respects_exclusions() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![IndexedPath::new("root_file", true)];
        index.populate(Some(vec![ExclusionPattern::new("**/directory").unwrap()]), true);

        assert_eq!(index.entries, expected_entries);
    }

    #[test]
    fn populate_lowercases_entries_when_case_sensitive_is_false() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![IndexedPath::new("directory/capitalized_file", false),
                                    IndexedPath::new("directory/nested_file", false),
                                    IndexedPath::new("root_file", false)];
        index.populate(None, false);
        index.entries.sort();

        assert_eq!(index.entries, expected_entries);
    }

    #[test]
    fn populate_lowercases_entries_when_case_sensitive_is_true() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![IndexedPath::new("directory/Capitalized_file", true),
                                    IndexedPath::new("directory/nested_file", true),
                                    IndexedPath::new("root_file", true)];
        index.populate(None, true);
        index.entries.sort();

        assert_eq!(index.entries, expected_entries);
    }

    #[test]
    fn find_defers_to_matching_module() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        index.populate(None, true);
        let term = "root";
        let limit = 5;

        // Get a string version of the results (PathBuf doesn't implement the display trait).
        let results: Vec<String> = index.find(term, limit)
                                        .iter()
                                        .map(|r| r.to_string_lossy().into_owned())
                                        .collect();

        assert_eq!(results, vec!["root_file".to_string()]);
    }
}
