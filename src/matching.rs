use std::path::PathBuf;
use std::cmp::Ordering;

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

/// Compares the query string to the subject,
/// and returns a score between 0 and 1.
pub fn similarity(query: &str, subject: &str) -> f32 {
    // Exact matches produce a perfect score.
    if query == subject {
        return 1.0;
    }

    let mut overall_score = 0.0;
    let subject_length = subject.chars().count();

    // We'll use these to add weight to consecutive character matches.
    let mut previous_match_indices = Vec::new();
    let mut current_match_indices  = Vec::new();

    for query_char in query.chars() {
        let mut character_score = 0.0;
        for (index, subject_char) in subject.chars().enumerate() {
            // For every occurrence of a query character in the
            // subject increase the individual character's score.
            if query_char == subject_char {
                character_score += 1.0;

                // Track the index at which we found a match,
                // so that we can detect subsequent matches.
                current_match_indices.push(index);

                // If the last query character matched the previous subject
                // character, there are at least two consecutive characters
                // that match; bump the character score to account for that.
                if index > 0 && previous_match_indices.contains(&(index-1)) {
                    character_score += 1.0;
                }
            }
        }

        // Limit the character score to a maximum value
        // of "1" and add it to the overall score.
        character_score /= subject_length as f32;
        overall_score += character_score;

        // If this query character doesn't exist in the subject,
        // penalize the overall score.
        if current_match_indices.is_empty() {
            overall_score -= 10.0;
        }

        // The current matches become the
        // previous ones for the next iteration.
        previous_match_indices = current_match_indices;
        current_match_indices = Vec::new();
    }

    // Return an overall score, limited to a maximum value of "1".
    (overall_score / subject_length as f32).max(0.0)
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

    #[test]
    fn similarity_correctly_scores_perfect_matches() {
        assert_eq!(similarity("src/hound.rs", "src/hound.rs"), 1.0);
    }

    #[test]
    fn similarity_correctly_scores_completely_different_terms() {
        assert_eq!(similarity("lib", "src"), 0.0);
    }

    #[test]
    fn similarity_scores_based_on_term_length() {
        let differing_length_score = similarity("houn", "houndhound");

        // Don't use a perfect match, since those product a perfect score.
        let same_length_score = similarity("houn", "hound");
        assert!(same_length_score > differing_length_score);
    }

    #[test]
    fn similarity_score_increases_for_consecutive_matches() {
        // Don't use a perfect match, since those product a perfect score.
        let properly_ordered_score = similarity("houn", "hound");

        let improperly_ordered_score = similarity("nuoh", "hound");
        assert!(properly_ordered_score > improperly_ordered_score);
    }

    #[test]
    fn similarity_score_decreases_for_non_matching_characters() {
        // Don't use a perfect match, since those product a perfect score.
        let non_matching_score = similarity("houns", "hound");

        let subset_score = similarity("houn", "hound");
        assert!(subset_score > non_matching_score);
    }
}
