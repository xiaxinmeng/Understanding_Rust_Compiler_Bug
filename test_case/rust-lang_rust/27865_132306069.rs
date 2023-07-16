 rust
const T: usize = 6;

pub struct BTreeMap<K, V> {
    root: Option<Box<Node<K, V>>>,
    ...
}

#[repr(C)]
struct Node<K, V> {
    keys: [K; 2 * T - 1],
    vals: [V: 2 * T - 1],
    parent: *mut InternalNode<K, V>,
    len: u16,
    idx: u16,
    is_leaf: bool,
}

#[repr(C)]
struct InternalNode<K, V> {
    node: Node<K, V>,
    edges: [*mut Node<K, V>; 2 * T],
}
