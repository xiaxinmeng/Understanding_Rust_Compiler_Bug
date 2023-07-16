rust
fn main() {
    || -> Result<u16, u64> {
        if true {
            return Err(42u64);
        }
        Ok(())
    };
}
