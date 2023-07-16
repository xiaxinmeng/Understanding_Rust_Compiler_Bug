rust
fn unwrap_zero(x: Result<u32, ZeroVariants>) -> u32 {
    match x {
        Ok(y) => y,
        Err(z) => match z {},
    }
}
