use std::path::PathBuf;
use std::cmp::Ordering;

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
/// use std::path::PathBuf;
/// use bloodhound::matching::find;
///
/// let entries = vec![
///     PathBuf::from("bloodhound.rs".to_string()),
///     PathBuf::from("lib.rs".to_string())
/// ];
/// let matches = find("lib", &entries, 1);
///
/// assert_eq!(matches[0].path.to_str().unwrap(), "lib.rs");
/// ```
pub fn find(needle: &str, haystack: &Vec<PathBuf>, max_results: usize) -> Vec<Result> {
    let mut results = Vec::new();

    // Calculate a score for each of the haystack entries.
    for path in haystack.iter() {
        results.push(Result{
            path: path.clone(),
            score: similarity(needle, path.to_str().unwrap())
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

/// Looks for space delimited terms in `query` that occur in `data`,
/// returning a score between 0 and 1, based on the percentage of letters covered
/// in data. Queries with terms that do not exist in `data` return a score of 0.
pub fn similarity(query: &str, data: &str) -> f32 {
    let mut score: f32 = 0.0;

    // Step through all of the query's terms.
    for term in query.split(" ") {
        let mut found = false;

        // Look for term matches in data.
        for (byte_index, _) in data.char_indices() {
            if data[byte_index..].starts_with(term) {
                // Match found; increase score relative to term size.
                score += term.len() as f32/data.len() as f32;

                // Track that we've found a match for this term.
                found = true;
            }
        }

        // Return zero whenever a query term cannot be found in data.
        if !found { return 0.0 }
    }

    // Overlapping matches can produce a score larger than 1.0. Normalize these values.
    score.min(1.0)
}

#[cfg(test)]
mod tests {
    use super::find;
    use std::path::PathBuf;

    #[test]
    fn find_returns_a_correctly_ordered_set_of_results() {
        let haystack = vec![
            PathBuf::from("src/hound.rs".to_string()),
            PathBuf::from("lib/hounds.rs".to_string()),
            PathBuf::from("Houndfile".to_string())
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
            PathBuf::from("src/hound.rs".to_string()),
            PathBuf::from("lib/hounds.rs".to_string()),
            PathBuf::from("Houndfile".to_string())
        ];
        let results = find("Hound", &haystack, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn similarity_scores_correctly_when_there_are_no_similarities() {
        assert_eq!(super::similarity("blood", "hound"), 0.0);
    }

    #[test]
    fn similarity_scores_correctly_when_there_is_an_exact_match() {
        assert_eq!(super::similarity("bloodhound", "bloodhound"), 1.0);
    }

    #[test]
    fn similarity_scores_correctly_when_there_is_a_half_match() {
        assert_eq!(super::similarity("blood", "bloodhound"), 0.5);
    }

    #[test]
    fn similarity_sums_term_matches() {
        assert_eq!(super::similarity("blood hound", "bloodhound"), 1.0);
    }

    #[test]
    fn similarity_limits_score_to_1() {
        assert_eq!(super::similarity("lol", "lololol"), 1.0);
    }

    #[test]
    fn similarity_returns_zero_when_there_are_unmatched_terms() {
        assert_eq!(super::similarity("bd hound", "bloodhound"), 0.0);
    }
}
