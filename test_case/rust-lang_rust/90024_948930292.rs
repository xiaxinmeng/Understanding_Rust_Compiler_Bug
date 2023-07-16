rust
pub struct Graph<N, E, Ix> {
    nodes: Vec<N>,
    edges: Vec<E>,
    ix: Vec<Ix>,
}
fn graph<N, E>() -> Graph<N, E, u32> {
    todo!()
}
fn main() {
    let g = graph();
    || g.hello()
}
