rust
match result {
    Ok(o) => o.report(),
    Err(err) => {
        drop(writeln!(io::stderr(), "Error: {err:?}"));
        err.report()
    }
}
