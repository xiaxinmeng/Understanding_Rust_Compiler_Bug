rust
#![feature(nll)]

enum Xyz {
    A,
    B,
}

fn main() {
    let mut e = Xyz::A;
    let f = &mut e;
    let g = f;
    match e {
        Xyz::A => println!("a"),
        Xyz::B => println!("b"),
    };
    *g = Xyz::B;
}
