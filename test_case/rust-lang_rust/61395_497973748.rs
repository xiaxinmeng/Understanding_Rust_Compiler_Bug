rust
#![feature(const_generics)]

struct Generic<const C: usize>;

fn foo(x : usize) {}

fn main() {
    let x = Generic::<0>;
    foo(x);
}
