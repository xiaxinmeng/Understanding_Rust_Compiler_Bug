 rust
use X::{A, B};

fn f() -> [uint, ..A] {
    unreachable!()
}

fn main() {
    let x: [uint, ..B] = f();
}
