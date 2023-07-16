rust
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;

const NEEDLE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const HAYSTACK: &'static [u8] = &[0; 10 * 1024 * 1024];

pub fn bar1() -> Result<(), Box<dyn Error>> {
    if HAYSTACK
        .windows(NEEDLE.len())
        .filter(|window| window == &NEEDLE)
        .count() > 0
    {
        println!("bar");
    }

    Ok(())
}

pub fn bar2() -> Result<bool, Box<dyn Error>> {
    if HAYSTACK
        .windows(NEEDLE.len())
        .filter(|window| window == &NEEDLE)
        .count() > 0
    {
        println!("bar");
    }

    Ok(true)
}

pub fn foo<P: AsRef<Path>>(_p: P) -> Result<Vec<HashMap<u32, u32>>, Box<dyn Error>> {
    //bar1()?;
    bar2()?;
    let ret = vec![];
    Ok(ret)
}
