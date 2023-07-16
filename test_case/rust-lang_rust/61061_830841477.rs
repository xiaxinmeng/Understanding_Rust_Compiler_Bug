rust
struct A;

#[must_use]
struct B;

fn t() -> (A, B) {
    (A, B)
}

fn main() {
    let (a, _b) = t();
}
