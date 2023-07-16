rust
fn unconstrained_return<T>() -> Result<T, String> {
    let ffi: fn() -> T = transmute(some_pointer);
    Ok(ffi())
}
fn foo() {
    match unconstrained_return::<_>() {
        Ok(x) => x,  // `x` has type `_`, which is unconstrained
        Err(s) => panic!(s),  // … except for unifying with the type of `panic!()`
        // so that both `match` arms have the same type.
        // Therefore `_` resolves to `!` and we "return" an `Ok(!)` value.
    };
}
