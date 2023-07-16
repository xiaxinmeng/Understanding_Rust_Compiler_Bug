rust
struct StrictHashSet<K: Hash> { ... }

struct MyType<E, C = StrictHashSet<E>> { ... }
