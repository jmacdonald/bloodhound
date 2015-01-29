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
    use super::*;
    use super::edit_distance;

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
