rust
#![feature(impl_trait_in_bindings)]
fn main() {
    // explicitly annotate the closure arg as &_.
    // same error as OP
    const func: impl Fn(&str) -> usize = |s: &_| s.parse().unwrap();

    // not even this helps
    const func: impl for<'a> Fn(&'a str) -> usize = |s: &_| s.parse().unwrap();
}
