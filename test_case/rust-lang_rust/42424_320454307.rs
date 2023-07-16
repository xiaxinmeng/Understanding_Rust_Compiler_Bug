rust
#![feature(catch_expr)]

pub struct X;

pub fn new_x() -> Result<X, ()> {
    return Ok(X);
}

fn main() {
    do catch {
        let x: X = new_x()?;
        Ok(())
    };
}
