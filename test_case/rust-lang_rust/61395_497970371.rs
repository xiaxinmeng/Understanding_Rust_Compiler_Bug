rust
#![feature(const_generics)]

struct Generic<const C: usize>;


fn main() {
    let x = Generic::<0>;
    let y = Generic::<1>;
    let _ = match 1 {
        0 => x,
        _ => y
    };
}
