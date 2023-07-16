rust
fn do_something() -> Result<(), Error> {
    let e1 = attempt("1").yeet_ok()?;
    let e2 = attempt("2").yeet_ok()?;
    let e3 = attempt("3").yeet_ok()?;
    Err(Error::All(vec![e1, e2, e3]))
}

fn do_something_option() -> Option<&'static str> {
    attempt_option("1").yeet_some()?;
    attempt_option("2").yeet_some()?;
    attempt_option("3").yeet_some()?;
    None
}
