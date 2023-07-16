 rust
extern crate timely;

use timely::example::*;
use timely::communication::*;
use timely::progress::nested::subgraph::new_graph;

fn triangles<C: Communicator>(communicator: C) {
    let mut computation = new_graph::<u64, C>(communicator);
    let (mut input, mut stream) = computation.new_input::<u32>();
}
