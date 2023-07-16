rust
struct InternedSet<'arena, T: Hash + Eq> {
    inner: ShardedHashMap<&'arena T, ()>
}

impl InternedSet<'arena, T: Hash + Eq> {
    pub fn intern<Q>(&self, value: Q, move_to_arena: impl FnOnce(Q) -> &'arena T) -> Uniq<'arena, T> { 
        // ... 
    }
}
