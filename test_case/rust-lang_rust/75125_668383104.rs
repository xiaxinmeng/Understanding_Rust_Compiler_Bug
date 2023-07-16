rust
#![feature(naked_functions)]

#[naked]
fn main() {
    println!("hey");
}
