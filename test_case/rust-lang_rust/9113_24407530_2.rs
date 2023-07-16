 rust
    pub fn find_or_fallback_with<'a,A,T>(
        &'a mut self,
        k: K,
        a: A,
        found: &fn(&K, &'a mut V, A) -> T,
        fallback: &fn(&'a mut HashMap<K, V>, &K, A) -> T) -> T {
        if self.size >= self.resize_at {
            // n.b.: We could also do this after searching, so
            // that we do not resize if this call to insert is
            // simply going to update a key in place.  My sense
            // though is that it's worse to have to search through
            // buckets to find the right spot twice than to just
            // resize in this corner case.
            self.expand();
        }

        let hash = k.hash_keyed(self.k0, self.k1) as uint;
        match self.bucket_for_key_with_hash(hash, &k) {
            TableFull => fail!("Internal logic error"),
            FoundEntry(idx) => {
                found(&k, self.mut_value_for_bucket(idx), a)
            }
            FoundHole(_) => {
                fallback(self, &k, a)
            }
        }
    }

