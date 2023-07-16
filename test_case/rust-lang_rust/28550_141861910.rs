 rust
// Using F directly causes it to compile.
struct A<F: FnOnce()>(F::Output);
struct B<F: FnOnce()>(A<F>);

// Removing Option causes it to compile.
fn foo<F: FnOnce()>(f: F) -> Option<B<F>> {
    panic!()
}

fn main() {
    || foo(||());
}
