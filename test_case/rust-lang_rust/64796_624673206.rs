rust
#![feature(cfg_version)]
fn main() {
    test();
}

#[cfg(version("1.45.0"))]
fn test() {
    println!("Yes")
}

#[cfg(not(version("1.45.0")))]
fn test() {
    println!("No")
}
