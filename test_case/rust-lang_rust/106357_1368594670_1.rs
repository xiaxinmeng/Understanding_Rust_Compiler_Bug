rust
fn breaks() -> Result<bool, i32> {
    match Err(1) {
         Ok(x) => x,
         Err(e) => return Err(e.into())
    };
}
