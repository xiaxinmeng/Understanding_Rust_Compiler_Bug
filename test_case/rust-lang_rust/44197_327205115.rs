Rust
fn multiply<G>(g: G, x: i32) -> impl Generator<Return = (), Yield = i32>
where
    G: Generator<Return = (), Yield = i32>,
{
    || while let GeneratorState::Yielded(v) = g.resume() {
        yield v * x;
    }
}
