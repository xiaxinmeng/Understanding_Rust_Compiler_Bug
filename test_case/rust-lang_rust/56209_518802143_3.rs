rust
pub struct SpecialIndexOperation<T> {
    data: T
    node_to_maybe_update: NodeIndex,
}
impl<T> AsRef<NodeIndex> for SpecialIndexOperation>T> {
    fn as_ref<'a>(&'a self) -> &'a NodeIndex {
        self.node_to_maybe_update
    }
}
