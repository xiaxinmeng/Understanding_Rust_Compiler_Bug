 rust
fn Specific() {}
type Specific = Generic<usize>;
fn f() {
    Specific(42) // Should `Specific` resolve to the function or the constructor?
}
