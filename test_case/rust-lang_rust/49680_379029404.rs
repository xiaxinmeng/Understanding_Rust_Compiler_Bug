rust
#[derive(PartialEq, Eq)]
struct S;

const C: S = S;

fn main() {
    let C = S; // OK
}
