rust
fn foo() -> Result<(), String>  {
    (try!(Ok::<u8, String>(1)));
    Ok(())
}

fn main() {}
