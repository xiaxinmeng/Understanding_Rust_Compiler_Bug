 rust
let p = Some(45).and_then({
    // This is a block with only one expression.
    // It returns this expression (the closure):
    |x| { Some(x * 2) } // braces optional
});
