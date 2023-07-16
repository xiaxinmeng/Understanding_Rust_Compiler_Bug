rust
pub unsafe fn qux(x: Result<String, String>) -> Result<String, String> {
    match x {
        Ok(s) => Ok(s),
        Err(s) => Err(s),
    }
}
