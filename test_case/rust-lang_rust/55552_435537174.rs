rust
extern crate rayon; // 1.0.2

struct A;
struct B;
struct C;

fn main() {
    let ((_a, _b), _c): (_, C) = rayon::join(
        || { (A, B) },
        || { C });
}
