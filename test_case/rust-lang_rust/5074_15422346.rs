 rust
enum Foo {
    A(int), B(int), C
}

fn main() {
    let mut x = A(5);
    let res = match x {
        A(ref mut a) => Some(a),
        B(ref mut b) => Some(b),
        C => None
    };
}
