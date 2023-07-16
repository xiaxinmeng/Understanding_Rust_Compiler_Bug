rust
#![feature(catch_expr)]

pub struct X;

pub fn new_x() -> Result<X, ()> {
    return Ok(X);
}

fn main() {
    let _: Result<_, ()> = do catch {   // <---------
        let x: X = new_x()?;
        Ok(())
    };
}
