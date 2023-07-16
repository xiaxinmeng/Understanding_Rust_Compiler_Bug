Rust
type X = impl Clone;
fn foo() -> X {
    |_| ()
}
