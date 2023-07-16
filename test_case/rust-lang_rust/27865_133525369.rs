 rust
struct BTreeMap<K, V> {
    height: usize,
    root: Option<Box<Node<K, V, height>>>
}

struct Node<K, V, height: usize> {
    keys: [K; 2 * T - 1],
    vals: [V; 2 * T - 1],
    edges: if height > 0 {
        [Box<Node<K, V, height - 1>>; 2 * T]
    } else { () }
    parent: *mut Node<K, V, height + 1>,
    len: u16,
    idx: u16,
}
