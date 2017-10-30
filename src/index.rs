use fragment::matching;
use ExclusionPattern;
use walkdir::{DirEntry, Error, WalkDir};
use std::path::PathBuf;
use std::clone::Clone;

pub struct Index {
    path: PathBuf,
    entries: Vec<IndexedPath>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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
            entries: Vec::new()
        }
    }

    /// Finds all files inside and beneath the index path
    /// and adds them to the index entries vector.
    pub fn populate(&mut self, exclusions: Option<Vec<ExclusionPattern>>) {
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
                    IndexedPath(PathBuf::from(entry_path))
                );
            });
        }
    }

    pub fn find(&self, term: &str, limit: usize) -> Vec<PathBuf> {
        matching::find(term, &self.entries, limit, true)
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
    extern crate fragment;

    use super::{Index, IndexedPath, ExclusionPattern};
    use std::path::PathBuf;

    #[test]
    fn populate_adds_all_files_to_entries() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![IndexedPath(PathBuf::from("directory/nested_file")),
                                    IndexedPath(PathBuf::from("root_file"))];
        index.populate(None);
        index.entries.sort();

        assert_eq!(index.entries, expected_entries);
    }

    #[test]
    fn populate_respects_exclusions() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        let expected_entries = vec![IndexedPath(PathBuf::from("root_file"))];
        index.populate(Some(vec![ExclusionPattern::new("**/directory").unwrap()]));

        assert_eq!(index.entries, expected_entries);
    }

    #[test]
    fn find_defers_to_matching_module() {
        let path = PathBuf::from("tests/sample");
        let mut index = Index::new(path);
        index.populate(None);
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
