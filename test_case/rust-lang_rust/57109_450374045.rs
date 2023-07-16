rust
fn to_value<V, T: MaybeResult<V> + Into<Option<V>>>(t: T) -> Option<V> {
    ...
}
