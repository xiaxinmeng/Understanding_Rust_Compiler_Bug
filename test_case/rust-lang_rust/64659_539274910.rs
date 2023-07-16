rust
// compile-flags: -Zsave-analysis

trait NodeIndexable { type NodeId; }
impl<T> NodeIndexable for T { type NodeId = (); }

fn main() {
    struct Data<T: NodeIndexable> {
        x: T::NodeId,
    }
}
