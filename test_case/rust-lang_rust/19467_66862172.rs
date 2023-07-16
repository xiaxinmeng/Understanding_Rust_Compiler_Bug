 rust
fn f<F>(_: F) where F: Fn() {}

fn main() {
    let closure = || {};   // currently, this defaults to a boxed closure
    f(closure);  // currently, errors with `Fn` trait is not implemented for `||` type
}
