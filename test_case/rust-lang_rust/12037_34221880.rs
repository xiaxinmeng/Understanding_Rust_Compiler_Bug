
fn foo() -> io::IoResult<()> {
    check!(some_function());

    let bar = check!(some_function());

    if check!(some_function()) {
        // ...
    }

    match check!(some_function()) {
        // ...
    }
}
