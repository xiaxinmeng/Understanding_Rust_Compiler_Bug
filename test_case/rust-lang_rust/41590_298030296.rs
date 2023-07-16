rust
    pub fn binary_search_by_key<'a, B, F, Q>(&'a self, b: &Q, f: F) -> Result<usize, usize>
        where F: FnMut(&'a T) -> B,
              B: Borrow<Q>,
              Q: Ord + ?Sized
