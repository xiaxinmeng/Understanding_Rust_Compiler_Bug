rust
#![feature(never_type)]

fn main() {
    let x: !;
    println!("asdf");
    x = panic!();
}
