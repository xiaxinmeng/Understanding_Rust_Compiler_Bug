rust
fn f() -> Result<(), ()> {
    Ok(())?; // <--
    Ok(())
}
