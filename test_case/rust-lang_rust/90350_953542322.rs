rust
struct B<const GRAPH_ID: usize>;

pub struct C<const GRAPH_ID: usize> {
    node_list: Vec<B<GRAPH_ID>>,
}

impl<const GRAPH_ID: usize> C<GRAPH_ID> {
    pub fn rem(mut self) {
        let _ = || self.node_list.remove(0);
    }
}

fn main() {}
