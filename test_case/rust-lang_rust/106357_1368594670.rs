rust
fn breaks() -> Result<bool, i32> {
    Err(1)?;
}
