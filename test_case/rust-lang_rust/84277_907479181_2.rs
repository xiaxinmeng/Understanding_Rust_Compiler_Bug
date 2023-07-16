rust
fn foo() -> MyResult<(), io::Error> {
    fs::File::open("foo.txt")?;
}

fn bar() -> MyResult<(), io::Error> {
    // I need bar to show up in error traces, so I wrap with Ok(..?).
    // Without my custom MyResult, I am unable to intercept this invocation of the `?` operator, because
    // the return type is the same as that of `foo`.
    Ok(foo()?)
}
