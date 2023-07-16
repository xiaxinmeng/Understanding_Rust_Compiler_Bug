rust
// bar.rs
extern crate foo;
fn main() {
    let foo = foo::Foo; //~ error[E0603]: unit struct `Foo` is private
}
