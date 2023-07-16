rust
fn main() -> Result<(), &'static str> {
    let a = 0usize;
    let b = &a;

    let _c: usize = {
        let d = &""
            .parse()
            .map_err(|_| "")?;
        b + d
    };

    Ok(())
}
