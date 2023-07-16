rust
fn multiply<G>(mut g: G, x: i32) -> impl Generator<Return = (), Yield = i32>
where
    G: Generator<Return = (), Yield = i32>
{
    move || loop {
        let state = g.resume();
        if let GeneratorState::Yielded(v) = state {
            yield v * x;
        }
        else {
            return;
        }
    }
}
