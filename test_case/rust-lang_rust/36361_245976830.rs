 rust
enum E { A, B, C }

fn main() {
    use E::*;
    let value = A;
    match value {
    A | B => {},
    C => {}
    }
}
