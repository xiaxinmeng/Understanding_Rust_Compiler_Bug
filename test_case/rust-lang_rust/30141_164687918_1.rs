 rust
#![feature(const_fn)]
const fn bla() -> i64 { 6 }
const FOO: i64 = bla();

fn main() {
    match 6 {
        FOO => println!("hi"),
        _ => println!("bye"),
    }
}
