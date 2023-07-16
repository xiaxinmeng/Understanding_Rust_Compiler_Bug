 rust
fn main() {
    let foo = || 5u;
    fn foo() -> uint { 42 }
    println!("{}", foo()); // prints 5
}
