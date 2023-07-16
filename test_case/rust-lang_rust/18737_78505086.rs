 rust
struct S { .. }
fn foo(x: Box<Any+Send>) -> bool {
    Any::is::<S>(&x)
}
