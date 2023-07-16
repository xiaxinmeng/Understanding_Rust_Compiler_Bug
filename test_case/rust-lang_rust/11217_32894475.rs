 rust
struct HashMapHashAlloc<K, V, Hash, Alloc> {
    ...
}

type HashMapHash<K, V, Hash> = HashMapHashAlloc<K, V, Hash, LibcAllocator>;
type HashMap<K, V> = HashMapHash<K, V, SipHash>;
