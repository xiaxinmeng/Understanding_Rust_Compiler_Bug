 rust
#![feature(default_type_parameter_fallback)]
#![feature(iter_arith)]

fn main() {
    let a = [1, 2, 3, 4, 5];
    let it = a.iter();
    assert_eq!(it.sum(), 15);
}
