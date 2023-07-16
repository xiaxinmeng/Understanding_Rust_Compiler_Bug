 rust
#![feature(const_fn)]
const fn foo(x: i32) -> i32 {
    return x;
}
fn main() {
    let x = foo(2);
    println!("{}", x);
}
