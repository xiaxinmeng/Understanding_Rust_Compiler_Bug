 rust
struct A;
impl Trait for A {
    fn method(&self) -> char { 'A' }
    const CONST: char = 'A';
}
struct B;
impl Trait for B {
    fn method(&self) -> char { 'B' }
    const CONST: char = 'B';
}
