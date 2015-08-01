use matching;
use std::path::PathBuf;
use std::collections::hash_map::HashMap;

pub struct Entry {
    pub path: PathBuf,
    pub index: HashMap<char, Vec<usize>>,
}

impl Entry {
    /// Compares the query string to the entry,
    /// and returns a score between 0 and 1.
    pub fn similarity(&self, query: &str) -> f32 {
        // Exact matches produce a perfect score.
        if query == self.path.to_string_lossy() {
            return 1.0;
        }

        let mut overall_score = 0.0;
        let path_length = self.path.chars().count();

        let mut last_char = ' ';

        for (query_char_index, query_char) in query.chars().enumerate() {
            let mut character_score = 0.0;

            // Look for the query's character in the path's index, bumping
            // the character score up for each occurrence in the path.
            match index.get(&query_char) {
                Some(occurrences) => character_score += occurrences.len() as f32,
                None => {
                    // If this query character doesn't exist in the path,
                    // penalize the overall score.
                    overall_score -= 10.0;
                },
            }

            // Check for consecutive character matches.
            if query_char_index > 0 {
                // Lookup the previous query character's matching indices in the
                // path; we'll check to see if they're the preceding character.
                match index.get(&last_char) {
                    Some(occurrences) => {
                        // If the last query character matched the previous path
                        // character, there are at least two consecutive characters
                        // that match; bump the character score to account for that.
                        if occurrences.contains(&(query_char_index-1)) {
                            character_score += 1.0;
                        }
                    },
                    None => (),
                }
            }

            // Limit the character score to a maximum value
            // of "1" and add it to the overall score.
            character_score /= path_length as f32;
            overall_score += character_score;

            // Track the current char so that we can check
            // for consecutive matches on the next iteration.
            last_char = query_char;
        }

        // Return an overall score, limited to a maximum value of "1".
        (overall_score / path_length as f32).max(0.0)
    }
}

fn index_path(path: &str) -> HashMap<char, Vec<usize>> {
    let mut index: HashMap<char, Vec<usize>> = HashMap::new();
    for (char_index, subject_char) in subject.chars().enumerate() {
        if index.contains_key(&subject_char) {
            match index.get_mut(&subject_char) {
               Some(occurrences) => occurrences.push(char_index),
               None => ()
            }
        } else {
           index.insert(subject_char, vec![char_index]);
        }
    }

    index
}

pub fn new(path: String) -> Entry {
    Entry{
        // Build the index before we transfer ownership of path.
        index: index_path(&path),
        path: PathBuf::from(path),
    }
}

mod tests {
    use super::new;

    #[test]
    fn similarity_correctly_scores_perfect_matches() {
        let entry = new("thing".to_string());
        assert_eq!(similarity("src/hound.rs", "src/hound.rs"), 1.0);
    }

    #[test]
    fn similarity_correctly_scores_completely_different_terms() {
        let entry = new("thing".to_string());
        assert_eq!(similarity("lib", "src"), 0.0);
    }

    #[test]
    fn similarity_scores_based_on_term_length() {
        let entry = new("thing".to_string());
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
