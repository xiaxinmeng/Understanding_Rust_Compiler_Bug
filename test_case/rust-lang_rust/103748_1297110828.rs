diff
   /// Returns `true` if `self` contains `elem`.
    #[inline]
    pub fn contains(&self, elem: T) -> bool {
-        assert!(elem.index() < self.domain_size);
+        if elem.index() >= self.domain_size { return false; }        
        let (word_index, mask) = word_index_and_mask(elem);
        (self.words[word_index] & mask) != 0
    }
