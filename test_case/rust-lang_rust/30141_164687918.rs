 rust
#![feature(const_fn)]
const BOO: (i32, i64) = (5, bla());
const fn bla() -> i64 { 6 }
const FOO: i64 = BOO.1;

fn main() {
    match 6 {
        FOO => println!("hi"),
        _ => println!("bye"),
    }
}
