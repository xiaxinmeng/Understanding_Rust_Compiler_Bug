 rust
// a.rs
fn foo() {}

// b.rs
extern mod a;
fn foo() { a::foo(); }

// c.rs
extern mod a;
extern mod b;
fn main() { b::foo(); }
