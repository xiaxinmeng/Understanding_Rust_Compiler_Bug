
#![feature(linkage)]
#[linkage = "external"]
pub static TEST2: bool = true;
fn main() {}
