rust
#![allow(unused)]

#[derive(Clone)]
struct Generate<T>(fn() -> T);

struct NonClone();

fn non_clone() -> NonClone {
    NonClone()
}

#[derive(Clone)]
struct CanClone();

fn can_clone() -> CanClone {
    CanClone()
}

fn main() {
    let g = Generate(non_clone);
    // g.clone(); // compile error!

    let g = Generate(can_clone);
    g.clone();
}
