rust
fn do_something() -> Result<(), Error> {
    let e1 = attempt("1").swap()?;
    let e2 = attempt("2").swap()?;
    let e3 = attempt("3").swap()?;
    Err(Error::all(vec![e1, e2, e3]))
}
