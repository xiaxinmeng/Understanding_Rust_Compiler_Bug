rust
#![feature(untagged_unions)]
fn main() {
    enum Void {}
    union A { a: (), v: Void }
    let a = A { a: () };
    match a.v {
    }
}
