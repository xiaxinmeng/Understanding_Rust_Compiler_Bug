rust
fn bug() -> Result<usize, Box<dyn Error>> {
    let result;

    if 1 < 4 {
        result = 4;
    } else {
        match Err("bla") {
            Ok(x) => x,
            Err(err) => return err.into(),
        }
    }

    Ok(result)
}
