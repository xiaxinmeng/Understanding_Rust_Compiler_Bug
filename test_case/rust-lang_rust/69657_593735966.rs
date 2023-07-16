rust
fn returns_err() -> Result<(), ()> {
    Err(())
}

fn match_postfix_op() -> Result<usize, ()> {
    let closure = || {
        let _ = returns_err()?;
        1
    };
    let i = closure()?;
    Ok(i)
}
