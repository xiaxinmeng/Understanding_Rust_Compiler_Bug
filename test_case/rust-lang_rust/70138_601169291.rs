rust
fn foo() -> Result<String, ()> {
    Err(())?
}
