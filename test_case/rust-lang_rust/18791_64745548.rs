 rust
#![feature(non_ascii_idents)]

struct C { θ: u8 }
fn main() {
    let x =  C { θ: 0 };
    (|c: C| c.θ )(x);
}
