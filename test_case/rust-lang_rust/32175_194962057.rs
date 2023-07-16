 rust
fn main() {
    match 42 {
        Err(err) => return Err(From::from(err)),
        Ok(val) => val,
    };
}
