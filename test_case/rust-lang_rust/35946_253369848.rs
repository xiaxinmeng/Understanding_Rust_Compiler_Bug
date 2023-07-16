 rust
fn does_not_return_result() -> i32 {
    try!(Ok(42));
}
