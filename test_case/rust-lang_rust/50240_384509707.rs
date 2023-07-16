
#[stable(feature = "btree_range", since = "1.17.0")]
pub struct Range<'a, K: 'a, V: 'a> {
    front: Handle<NodeRef<marker::Immut<'a>, K, V, marker::Leaf>, marker::Edge>,
    back: Handle<NodeRef<marker::Immut<'a>, K, V, marker::Leaf>, marker::Edge>,
}
