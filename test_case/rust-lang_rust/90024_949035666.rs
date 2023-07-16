rust
pub struct Graph<N, E, Ix> {
    _edges: E,
    _nodes: N,
    _ix: Vec<Ix>,
}
fn graph<N, E>() -> Graph<N, E, i32> {
    todo!()
}
fn main() {
    let g = graph::<i32, i32>();
    let _ = || g;
}
