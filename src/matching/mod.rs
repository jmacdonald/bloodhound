pub mod entry;
mod fragment;

use std::path::PathBuf;
use std::cmp::Ordering;
use matching::entry::Entry;

#[derive(Debug, PartialEq)]
pub struct Result {
    pub path: PathBuf,
    score: f32,
}

/// Given a set of path entries, `find` returns a set of `Result` objects
/// ordered by increasing score values (first values are closest matches).
/// If the result set is larger than `max_results`, the set is reduced to
/// that size.
///
/// # Examples
///
/// ```rust
/// use bloodhound::matching::entry;
/// use std::path::PathBuf;
/// use bloodhound::matching::find;
///
/// let entries = vec![
///     entry::new("bloodhound.rs".to_string()),
///     entry::new("lib.rs".to_string())
/// ];
/// let matches = find("lib", &entries, 1);
///
/// assert_eq!(matches[0].path.to_str().unwrap(), "lib.rs");
/// ```
pub fn find(needle: &str, haystack: &Vec<Entry>, max_results: usize) -> Vec<Result> {
    let mut results = Vec::new();

    // Calculate a score for each of the haystack entries.
    for entry in haystack.iter() {
        results.push(Result{
            path: entry.path.clone(),
            score: entry.similarity(needle)
        });
    }

    // Sort the results in ascending order (higher values are worse).
    results.sort_by(|a, b| {
        if a.score > b.score {
            Ordering::Less
        } else if a.score < b.score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // Make sure we don't exceed the specified result limit.
    results.truncate(max_results);

    results
}

#[cfg(test)]
mod tests {
    use super::entry;
    use super::find;
    use std::path::PathBuf;

    #[test]
    fn find_returns_a_correctly_ordered_set_of_results() {
        let haystack = vec![
            entry::new("src/hound.rs".to_string()),
            entry::new("lib/hounds.rs".to_string()),
            entry::new("Houndfile".to_string())
        ];
        let expected_results = vec![PathBuf::from("Houndfile"), PathBuf::from("src/hound.rs")];
        let results = find("Hound", &haystack, 2);
        for i in 0..2 {
            assert_eq!(results[i].path, expected_results[i]);
        }
    }

    #[test]
    fn find_returns_a_correctly_limited_set_of_results() {
        let haystack = vec![
            entry::new("src/hound.rs".to_string()),
            entry::new("lib/hounds.rs".to_string()),
            entry::new("Houndfile".to_string())
        ];
        let results = find("Hound", &haystack, 2);
        assert_eq!(results.len(), 2);
    }
}
