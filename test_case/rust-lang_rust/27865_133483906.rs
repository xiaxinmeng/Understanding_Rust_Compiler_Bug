 rust
const T: usize = 6;

pub struct BTreeMap<K, V> {
    root: Option<Box<Node<K, V>>>,
    ...
}

struct Node<K, V> {
    keys: [K; 2 * T - 1],
    vals: [V: 2 * T - 1],
    parent: *mut InternalNode<K, V>,
    len: u16,
    idx: u16,
}

struct InternalNode<K, V> {
    node: Node<K, V>,
    edges: EdgeData<K, V>,
}

enum EdgeData<K, V> {
    Leaves([Box<Node<K, V>>; 2 * T]),
    Internals([Box<InternalNode<K, V>>; 2 * T]),
}
