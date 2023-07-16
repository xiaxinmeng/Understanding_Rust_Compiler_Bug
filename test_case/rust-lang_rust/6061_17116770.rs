 rust
// Allow direct chaining with `task_rng`
impl<R: Rng> Rng for @R {
    fn next(&self) -> u32 { (*self).next() }
}
