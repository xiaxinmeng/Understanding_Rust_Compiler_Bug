 rust
use linearscan::Graph;

mod linearscan {

  mod graph {
    pub struct Graph { block_id: uint, }
    impl Graph {
      pub fn new() -> Graph { fail!() }
    }
  }

  impl Graph {}
}

pub fn main() {
  Graph::new();
}
