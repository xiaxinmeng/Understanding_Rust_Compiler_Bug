rust
impl SadBee for for<'a> fn(&'a ()) {
    const ASSOC: usize = 0;
}
// This `impl` is more generic than your but still covers fn(`&'static ()`)
// (and any other `fn(&'concrete_lifetime U)`)
impl<T> SadBee for fn(T) {
    const ASSOC: usize = 100;
}
