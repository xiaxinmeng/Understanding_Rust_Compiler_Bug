 rust
struct A;

impl A {
    fn foo(&mut self) {}
}

fn main() {
    let mut a = A;
    (a).foo();
}
