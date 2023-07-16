rust
use std::convert::Infallible;

struct MyStruct;

impl From<Infallible> for MyStruct {
    fn from(val: Infallible) -> MyStruct {
        match val {}
    }
}

fn make_ok() -> Result<(), Infallible> {
    Ok(())
}

fn foo() -> Result<bool, MyStruct> {
    make_ok()?;
    Ok(true)
}

fn main() {
    let _ = foo();
}
