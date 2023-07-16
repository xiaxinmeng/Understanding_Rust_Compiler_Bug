rust
fn unconstrained_return<T>() -> Result<T, String> {
    let ffi: fn() -> T = transmute(some_pointer);
    Ok(ffi())
}
fn foo() {
    match unconstrained_return::<_>() {
        Ok(x) => x,  // `x` has type `_`, which is constrained to be `Inhabited`
        Err(s) => panic!(s),  // the type of `panic!()` is never type
        // `x` cannot be never type since it is `Inhabited`, so it will fallback to `()`.
        // Therefore `_` resolves to `()` and we "return" an `Ok(())` value.
    };
}
