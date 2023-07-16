rust
fn foo() -> Result<i32, MyError> {
    let a = method_returning_option_i32()?;
    // ...
}
