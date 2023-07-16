rust
#![feature(linkage)]
  
#[linkage = "weak"]
pub fn fn1(a: u32, b: u32, c: u32) -> u32 {
    a + b + c
}

#[linkage = "weak"]
pub fn fn2(a: u32, b: u32, c: u32) -> u32 {
    a + b + c
}
