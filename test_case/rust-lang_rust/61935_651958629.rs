rust
#![feature(const_generics)]
fn foo<const B: bool>() {}

fn bar<const N: usize>() {
    foo::<{ N == 0 }>();
}
