rust
fn f2<T>() -> Result<T, String> {
    unimplemented!()
}

fn f1() -> Result<i64, String> {
    match f2() {
        Ok(value) => Ok(value as i64),
        err => err,
    }
}
