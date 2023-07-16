rust
#![feature(const_generics)]

struct Generic<const C: usize>;

fn foo(x : Generic::<1>) {}

fn main() {
    let x = Generic::<0>;
    foo(x);
}
