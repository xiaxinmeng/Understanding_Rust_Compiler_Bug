 rust
impl<T, V> Predicate<V> for T where T: FnMut { .. }
impl<V> Predicate<V> for SomeMatcher<V> { .. } // What if `SomeMatcher<V>` implements `FnMut`?
