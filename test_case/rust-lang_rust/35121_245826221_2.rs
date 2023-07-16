 rust
enum Foo {
    Flim,
    Flam,
}

let result_foo: Result<T, Foo> = ...;
match result_foo {
    Ok(t) => ...,
    Err(Flim) => ...,
    Err(Flam) => ...,
}
