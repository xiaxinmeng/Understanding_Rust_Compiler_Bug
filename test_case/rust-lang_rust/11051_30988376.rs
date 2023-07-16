 rust
#[deriving(Clone, Eq, Ord)]
enum Node<K, V> {
    LeafNode(Leaf<K, V>),
    BranchNode(Branch<K, V>)
}
