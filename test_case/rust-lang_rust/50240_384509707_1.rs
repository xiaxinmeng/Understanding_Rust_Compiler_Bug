
pub enum Range<'a, K: 'a, V: 'a> {
    Empty,
    NonEmpty {
        front: Handle<NodeRef<marker::Immut<'a>, K, V, marker::Leaf>, marker::Edge>,
        back: Handle<NodeRef<marker::Immut<'a>, K, V, marker::Leaf>, marker::Edge>,
    },
}
