Rust
#![feature(const_generics)]

struct Generic<const V: usize>;

fn main() {
    let _ = Generic::<{0}>{};
}
