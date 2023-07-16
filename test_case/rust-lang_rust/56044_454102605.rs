rust
fn bindings_with_let(a: (LogDrop, LogDrop), b: (LogDrop, LogDrop)) {
    // Drop order in foo is the same as the following bindings.
    // _temp2 is declared after _x to avoid a difference between `_: T` and
    // `x: T` in function parameters.
    let _temp1 = a;
    let (_x, _) = _temp1;

     let _temp2 = b;
    let (_, _y) = _temp2;
}
