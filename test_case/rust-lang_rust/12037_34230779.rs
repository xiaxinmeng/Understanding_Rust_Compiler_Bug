 rust
fn foo() -> io::IoResult<()> {
    result!(some_function());

    let bar = result!(some_function());

    if result!(some_function()) {
        // ...
    }

    match result!(some_function()) {
        // ...
    }
}
