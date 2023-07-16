rust
#![feature(try_blocks)]

pub fn f() -> Result<i32, i32> {
    let collapse = try {
        g()?;
        h()?;
    };
    Ok(3)
}

fn g() -> Result<(), i32> { Err(0) }

fn h() -> Result<(), i32> { Err(1) }

fn main() { f().unwrap(); }
