rust
#![feature(impl_trait_in_bindings)]
const _: impl Fn() = async { 0 };
fn main() {}
