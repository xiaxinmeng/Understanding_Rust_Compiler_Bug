rust
fn loop_break_void() -> i32 {
    let loop_value = loop { break get_void() };
    // used to error because typeck thought the function
    // could return and thus need to coerce `()` into `i32`
    // right here.
}
