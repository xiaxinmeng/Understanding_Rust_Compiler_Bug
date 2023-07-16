rust
#![feature(nll)]

struct X {
    a: String,
    b: String,
}

fn main() {
    let x = X {
        a: String::new(),
        b: String::new(),
    };
    let &X { a, b } = &x;
}
