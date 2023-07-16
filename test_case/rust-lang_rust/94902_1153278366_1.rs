rust
fn generic<T>() -> Result<T, usize>
{
    panic!()
}

fn concrete() -> Result<usize, usize> {
    generic()?;

    Ok(0)
}

fn main() {
    concrete().unwrap();
}
