use matching::fragment;
use matching::fragment::Fragment;
use std::path::PathBuf;
use std::collections::hash_map::HashMap;

#[derive(PartialEq)]
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

        // Pre-calculate the path length as we'll be using it frequently.
        let path_length = self.path.to_string_lossy().chars().count();

        // We track fragment/substring matches, which have a greater weight than
        // a simple sum of individual character occurrences in the entry path.
        let mut match_fragments: Vec<Fragment> = Vec::new();

        // This counter is used to track characters in the query that aren't in
        // the entry's path, which has discrete weighting in the final score.
        let mut non_existent_char_count = 0;

        // Look for the query's character in the path's index.
        for query_char in query.chars() {
            match self.index.get(&query_char) {
                Some(occurrences) => {
                    // Initially, we'll assume that none of the occurrences
                    // of this character have been tracked as fragments.
                    let mut unaccounted_occurrences = occurrences.clone();

                    // Extend any existing fragment matches that also
                    // have this char at the end of their existing match.
                    for fragment in match_fragments.iter_mut() {
                        // Get the index at which this character would have to
                        // occur to extend this particular fragment over it.
                        let target_index = fragment.next_index();

                        if occurrences.contains(&target_index) {
                            // Since this character match is covered by this
                            // fragment, remove it from the unaccounted set.
                            match unaccounted_occurrences.iter().position(|&o| o == target_index) {
                                Some(position) => {
                                    unaccounted_occurrences.remove(position);
                                },
                                None => (),
                            }

                            // Bump the fragment's length to cover this char.
                            fragment.increase_length();
                        }
                    }

                    // Create fragment matches for any unaccounted occurrences.
                    for occurrence_index in unaccounted_occurrences.iter() {
                        match_fragments.push(fragment::new(*occurrence_index));
                    }

                },
                None => {
                    // Characters in the query string that
                    // aren't in the path reduce the score;
                    // track these so we can consider them later.
                    non_existent_char_count += 1;
                },
            }
        }

        // Determine the percentage of characters in the query string that
        // are in the entry, using the non-existent count we've calculated.
        let non_existence_penalty =
            // Guard against a potential arithmetic overflow here.
            if non_existent_char_count >= path_length {
                return 0.0f32
            } else {
                (path_length - non_existent_char_count) as f32 / path_length as f32
            };

        // Calculate an exponentially-scaled score based on fragment lengths.
        let fragment_score = match_fragments.iter().fold(0, |acc, ref fragment| {
            acc + fragment.length.pow(2)
        });

        // Calculate and return the similarity value. The path_length division
        // is used to offset the increased fragment score probability for
        // larger entry paths.
        fragment_score as f32 * non_existence_penalty / path_length as f32
    }
}

fn index_path(path: &str) -> HashMap<char, Vec<usize>> {
    let mut index: HashMap<char, Vec<usize>> = HashMap::new();
    for (char_index, path_char) in path.chars().enumerate() {
        if index.contains_key(&path_char) {
            match index.get_mut(&path_char) {
               Some(occurrences) => occurrences.push(char_index),
               None => ()
            }
        } else {
           index.insert(path_char, vec![char_index]);
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

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn similarity_correctly_scores_perfect_matches() {
        let entry = new("src/hound.rs".to_string());
        assert_eq!(entry.similarity("src/hound.rs"), 1.0);
    }

    #[test]
    fn similarity_correctly_scores_completely_different_terms() {
        let entry = new("src".to_string());
        assert_eq!(entry.similarity("lib"), 0.0);
    }

    #[test]
    fn similarity_scores_based_on_term_length() {
        let long_entry = new("hound library".to_string());
        let differing_length_score = long_entry.similarity("houn");

        // Don't use a perfect match, since those product a perfect score.
        let short_entry = new("hound".to_string());
        let same_length_score = short_entry.similarity("houn");

        assert!(same_length_score > differing_length_score);
    }

    #[test]
    fn similarity_score_increases_for_consecutive_matches() {
        let entry = new("hound".to_string());

        // Don't use a perfect match, since those product a perfect score.
        let properly_ordered_score = entry.similarity(" houn");

        let improperly_ordered_score = entry.similarity("nuoh");
        assert!(properly_ordered_score > improperly_ordered_score);
    }

    #[test]
    fn similarity_score_decreases_for_non_matching_characters() {
        let entry = new("hound".to_string());

        // Don't use a perfect match, since those product a perfect score.
        let non_matching_score = entry.similarity("houns");

        let subset_score = entry.similarity("houn");
        assert!(subset_score > non_matching_score);
    }

    #[test]
    fn similarity_score_is_zero_for_larger_query_with_no_matching_characters() {
        let entry = new("amp".to_string());
        assert_eq!(entry.similarity("hound"), 0.0f32);
    }
}
