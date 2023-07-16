
    fn find_or_insert(&mut self, k: K, v: V) -> &self/V {...}
    fn find_or_insert_with(&mut self, k: K, f: fn(&K) -> V) -> &self/V {...}
