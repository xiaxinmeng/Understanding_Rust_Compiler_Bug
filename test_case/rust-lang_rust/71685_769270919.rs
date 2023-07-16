
fn fn2(s: &str) -> Result<(), ()> {
    let s = s.parse().map_err(|_| ())?;
    Ok(())
}
