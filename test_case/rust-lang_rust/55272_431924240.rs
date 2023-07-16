rust
#![feature(impl_trait_in_bindings)]
fn main() {
    const func: impl Fn(String) -> usize = |s| s.parse().unwrap();
}
