rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let bar = foo()?;
    println!("{bar}");
    Ok(())
}

fn foo<T: Default>() -> Result<T, Box<dyn Error>> {
    Ok(T::default())
}
