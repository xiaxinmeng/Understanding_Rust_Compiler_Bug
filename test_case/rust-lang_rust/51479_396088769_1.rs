
fn f1() -> Result<i64, String> {
    Ok(f2()? as i64)
}
