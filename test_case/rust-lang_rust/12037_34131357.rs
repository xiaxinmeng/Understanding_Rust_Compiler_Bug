
fn foo() -> io::IoResult<()> {
    try!(some_function());

    let bar = try!(some_function());

    if try!(some_function()) {
        // ...
    }

    match try!(some_function()) {
        // ...
    }
}
