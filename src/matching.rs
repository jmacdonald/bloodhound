use std::path::PathBuf;
use std::cmp::Ordering;
use std::collections::hash_map::HashMap;

#[derive(Debug, PartialEq)]
pub struct Result {
    pub path: PathBuf,
    score: f32,
}

/// Given a set of paths, `find` returns a set of `Result` objects
/// ordered by increasing score values (first values are closest matches).
/// If the result set is larger than `max_results`, the set is reduced to
/// that size.
///
/// # Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use bloodhound::matching::find;
///
/// let paths = vec![PathBuf::from("bloodhound.rs"), PathBuf::from("lib.rs")];
/// let matches = find("lib", &paths, 1);
///
/// assert_eq!(matches[0].path.to_str().unwrap(), "lib.rs");
/// ```
pub fn find(needle: &str, haystack: &Vec<PathBuf>, max_results: usize) -> Vec<Result> {
    let mut results = Vec::new();

    // Calculate a score for each of the haystack entries.
    for path in haystack.iter() {
        match path.to_str() {
            Some(path_string) => {
                results.push(Result{
                    path: path.clone(),
                    score: similarity(needle, path_string)
                });
            },
            None => (),
        }
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
    use super::find;
    use super::similarity;
    use std::path::PathBuf;

    #[test]
    fn find_returns_a_correctly_ordered_set_of_results() {
        let haystack = vec![PathBuf::from("src/hound.rs"),
            PathBuf::from("lib/hounds.rs"), PathBuf::from("Houndfile")];
        let expected_results = vec![PathBuf::from("Houndfile"), PathBuf::from("src/hound.rs")];
        let results = find("Hound", &haystack, 2);
        for i in 0..2 {
            assert_eq!(results[i].path, expected_results[i]);
        }
    }

    #[test]
    fn find_returns_a_correctly_limited_set_of_results() {
        let haystack = vec![PathBuf::from("src/hound.rs"),
            PathBuf::from("lib/hounds.rs"), PathBuf::from("Houndfile")];
        let results = find("Hound", &haystack, 2);
        assert_eq!(results.len(), 2);
    }
}
