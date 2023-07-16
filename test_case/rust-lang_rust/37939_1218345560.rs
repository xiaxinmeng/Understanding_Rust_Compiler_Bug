rust
#![feature(try_blocks)]

pub fn result_nop_match(x: Result<i32, u32>) -> Result<i32, u32> {
    match x {
        Ok(x) => Ok(x),
        Err(x) => Err(x),
    }
}

pub fn result_nop_traits(x: Result<i32, u32>) -> Result<i32, u32> {
    try {
        x?
    }
}
