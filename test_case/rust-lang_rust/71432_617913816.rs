rust
fn foo() -> io::Result<()> { ... }
fn main() {
    if let Ok(x) = foo() { ... }
}
