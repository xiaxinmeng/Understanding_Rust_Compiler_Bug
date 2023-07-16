
#![feature(try_blocks)]

pub struct X;

pub fn new_x() -> Result<X, ()> {
    return Ok(X);
}

fn main() {
    try {
        let x: X = new_x()?;
        Ok(())
    };
}
