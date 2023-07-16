rust
fn err_semicolon() -> Result<bool, i32> {
    Err(1)?; // compile error, wrong type
}
fn err_no_semicolon() -> Result<bool, i32> {
    Err(1)? // works
}
fn yeet_semicolon() -> Result<bool, i32> {
    do yeet 1; // works
}
fn yeet_no_semicolon() -> Result<bool, i32> {
    do yeet 1 // works
}
