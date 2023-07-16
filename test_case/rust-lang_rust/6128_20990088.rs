
use std::hashmap::HashMap;

trait Graph<Node, Edge> {
    fn f(&self, Edge);

}

impl<E> Graph<int, E> for HashMap<int, int> {
    fn f(&self, _e: E) {
        fail!();
    }
}

fn main() {
    let g : ~HashMap<int, int> = ~HashMap::new();
    let _g2 : ~Graph<int,int> = g as ~Graph<int,int>;
}
