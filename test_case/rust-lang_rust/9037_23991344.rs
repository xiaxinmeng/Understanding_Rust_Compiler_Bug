 rust
fn foo() -> uint { 1 }
fn main() {
    let x: int = foo(); // should be `foo() as int`
}
