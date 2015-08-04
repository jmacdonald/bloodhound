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

        let path_length = self.path.to_string_lossy().chars().count();

        let mut match_fragments: Vec<Fragment> = Vec::new();
        let mut non_existent_char_count = 0;

        for query_char in query.chars() {
            // Look for the query's character in the path's index, bumping
            // the character score up for each occurrence in the path.
            match self.index.get(&query_char) {
                Some(occurrences) => {
                    // Initially, we'll assume that none of the occurrences
                    // of this character have been tracked as fragments.
                    let mut unaccounted_occurrences = occurrences.clone();

                    // Grow any fragment matches that also match this char.
                    for fragment in match_fragments.iter_mut() {
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
                    non_existent_char_count += 1;
                },
            }
        }

        let percent_existent = (
            path_length - non_existent_char_count) as f32 / path_length as f32;

        match_fragments.iter().fold(0, |acc, ref fragment| {
            acc + fragment.length.pow(2)
        }) as f32 * percent_existent / path_length as f32
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
}
