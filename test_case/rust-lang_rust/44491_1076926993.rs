rust
// in crate `upstream`
pub struct Set<K: Hash + Eq>(HashSet<K>);

// in crate `downstream`
enum MaybeSet<K> {
    Single(K),
    Set(upstream::Set<K>),   // Error: K requires Hash + Eq
}
