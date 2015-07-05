use std::path::PathBuf;

pub struct Result {
    pub path: PathBuf,
    score: usize,
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
/// let matches = find("lib", paths, 1);
///
/// assert_eq!(matches[0].path.to_str().unwrap(), "lib.rs");
/// ```
pub fn find(needle: &str, haystack: Vec<PathBuf>, max_results: usize) -> Vec<Result> {
    let mut results = Vec::new();

    // Calculate a score for each of the haystack entries.
    for path in haystack.iter() {
        results.push(Result{ path: path.clone(), score: edit_distance(needle, path.as_path().to_str().unwrap()) });
    }

    // Sort the results in ascending order (higher values are worse).
    results.sort_by(|a, b| a.score.cmp(&b.score));

    // Make sure we don't exceed the specified result limit.
    results.truncate(max_results);

    results
}

/// Determines the minimum number of edits required to transform
/// the first string into the second, using a matrix-driven
/// version of the Levenshtein distance algorithm.
///
fn edit_distance(first: &str, second: &str) -> usize {
    // Initialize a matrix we'll use to track the
    // edit distances as we step through both strings.
    let width = first.chars().count()+1;
    let height = second.chars().count()+1;
    let mut distances: Vec<usize> = Vec::with_capacity(width*height);

    // We want to be able to index values immediately,
    // without having to push data (usizes default to zero).
    unsafe { distances.set_len(width*height); }

    // Initialize first row and column values, when we come
    // across them, which is the distance from nothing to
    // every zero-index substring of the other word.
    // e.g. ("" -> "hou", "" -> "houn", "" -> "hound")
    for row in (0..height) {
        distances[row*width] = row;
    }
    for column in (0..width) {
        distances[column] = column;
    }

    // We need to manage indices manually; strings don't have an
    // index + character iterator, since indexing UTF-8 strings
    // doesn't work beyond the ASCII set. We, however, need to track
    // each substring combination, so they're useful in this scenario.
    let mut row = 1;
    let mut column = 1;
    for row_char in second.chars() {
        for column_char in first.chars() {
            if column_char == row_char {
                // When the strings share a common character, there's
                // no additional cost; we carry forward the optimum edit
                // distance we calculated without having considered it.
                distances[row*width + column] = distances[(row-1)*width + (column-1)];
            } else {
                distances[row*width + column] = *[
                    // Cost of removing a character from the first word.
                    distances[(row-1)*width + column] + 1,

                    // Cost of adding a character to the first word.
                    distances[row*width + (column-1)] + 1,

                    // Cost of substituting the character for another.
                    distances[(row-1)*width + (column-1)] + 1
                ].iter().min().unwrap();
            }
            column += 1;
        }
        row += 1;
        column = 1;
    }

    // The optimal edit distance is always in the
    // lower-right corner of the distance matrix.
    return distances[width*height-1]
}

#[cfg(test)]
mod tests {
    use super::find;
    use super::edit_distance;
    use std::path::PathBuf;

    #[test]
    fn find_returns_a_correctly_ordered_set_of_results() {
        let haystack = vec![PathBuf::from("src/hound.rs"),
            PathBuf::from("lib/hounds.rs"), PathBuf::from("Houndfile")];
        let expected_results = vec![PathBuf::from("Houndfile"), PathBuf::from("src/hound.rs")];
        let results = find("Hound", haystack, 2);
        for i in 0..2 {
            assert_eq!(results[i].path, expected_results[i]);
        }
    }

    #[test]
    fn find_returns_a_correctly_limited_set_of_results() {
        let haystack = vec![PathBuf::from("src/hound.rs"),
            PathBuf::from("lib/hounds.rs"), PathBuf::from("Houndfile")];
        let results = find("Hound", haystack, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn edit_distance_is_correct_for_removing_a_character() {
        assert_eq!(edit_distance("hound", "hounds"), 1);
    }

    #[test]
    fn edit_distance_is_correct_for_adding_a_character() {
        assert_eq!(edit_distance("hounds", "hound"), 1);
    }

    #[test]
    fn edit_distance_is_correct_for_substitutions() {
        assert_eq!(edit_distance("hound", "mound"), 1);
    }
}
