 rust
enum A { Outer(B) }
enum B { Inner1, Inner2, Inner3, }
fn main() { match A::Outer(B::Inner1) {A::Outer(B::Inner1) => {}} }
