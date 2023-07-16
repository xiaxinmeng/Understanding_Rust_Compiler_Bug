rust
#![allow(warnings)]
fn read(_: &i32) { }
fn main() {
    let mut x = 22;
    let p = &x;
    x += 1; // point A
    read(p); // point B
}
