 rust
extern crate timely;

use timely::progress::Graph;
use timely::example::stream::Stream;

pub trait Test<G: Graph> {
    fn test(&mut self) -> Stream<G, u64>;
}

impl<G: Graph> Test<G> for () {
    fn test(&mut self) -> Stream<G, u64> {

    }
}
