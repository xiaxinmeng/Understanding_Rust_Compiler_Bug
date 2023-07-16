
/// A Raw node handles the common code between Internal and Leaf nodes. Generally, it does all the
/// work assuming the Node is Internal, and for Leaf nodes the edge manipulation should just get
/// optimized away because of how empty types are treated.
struct RawNode<K, V, E> {
    length: uint,
    elems: *mut K,
    marker:   marker::CovariantType<(K, V, E)>,
}

pub struct InternalNode<K, V> {
    raw: RawNode<K, V, Node<K,V>>,
}

pub struct LeafNode<K, V> {
    raw: RawNode<K, V, ()>,
}

pub enum Node<K, V> {
    Internal(InternalNode<K, V>),
    Leaf(LeafNode<K, V>),
}
