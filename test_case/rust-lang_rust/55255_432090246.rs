rust
#![feature(fn_traits)]

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let tuple = (1, 3);
    println!("{}", add.call(tuple));
}
