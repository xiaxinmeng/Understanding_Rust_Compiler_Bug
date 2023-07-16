rust
match f2() {
    Ok(value) => Ok(value as i64),
    Err::<i32, String>(err) => Err::<i64, String>(err),
}
