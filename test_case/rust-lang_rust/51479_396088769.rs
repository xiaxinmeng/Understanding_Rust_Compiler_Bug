
fn f1() -> Result<i64, String> {
    f2().map(|v| v as i64)
}
