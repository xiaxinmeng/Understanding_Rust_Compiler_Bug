rust
fn get_first_yield<T>(mut gen: impl Generator<Yield = T>) -> Option<T> {
    // We know `resume` wasn't called on `gen` before because the caller needs to
    // move `gen` to call this method. Since we now own `gen` and we won't move it
    // anymore, it is safe for us to call `resume`.
    match unsafe { gen.resume() } {
        GeneratorState::Yielded(value) => Some(value),
        GeneratorState::Complete(_) => None,
    }
    // `gen` gets dropped here, therefore we are sure it isn't moved in the future.
}
