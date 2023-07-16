rust
fn fn2(s: &str) -> Result<(), ()> {
    let _ = s.parse().map_err(|_| ())?;
    Ok(())
}
