 rust
fn find<'a, Q>(&'a self, k: &Q) -> Option<&'a V>
    where Q: Hash<S>, () : Equiv<K, Q>
