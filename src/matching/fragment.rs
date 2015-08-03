/// Match fragments represent a set of
/// consecutive characters common to both terms. 
pub struct Fragment {
    pub length: usize,
    index: usize,
}

impl Fragment {
    /// The index _after_ those already part of the matching fragment. 
    /// Used by the matching algorithm to check for a match at the returned
    /// index, after which the length of the fragment will be increased.
    pub fn next_index(&self) -> usize {
        self.index + self.length
    }

    /// Following a successful match at `next_index`, this
    /// is used to increment the fragment's length by one.
    pub fn increase_length(&mut self) {
        self.length += 1;
    }
}

pub fn new(index: usize) -> Fragment {
    Fragment{ length: 1, index: index }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    pub fn next_index_returns_initial_index_plus_length() {
        assert_eq!(new(10).next_index(), 11);
    }

    #[test]
    pub fn increase_length_increments_next_index_value() {
        let mut fragment = new(10);
        fragment.increase_length();
        assert_eq!(fragment.next_index(), 12);
    }
}
