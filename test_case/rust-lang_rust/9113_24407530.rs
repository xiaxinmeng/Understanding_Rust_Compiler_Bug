 rust
    pub fn find_or_fallback_with<'a,A,T>(
        &'a mut self,
        k: K,
        a: A,
        found: &fn(&K, &'a mut V, A) -> T,
        fallback: &fn(&'a mut HashMap<K, V>, &K, A) -> T) -> T;
