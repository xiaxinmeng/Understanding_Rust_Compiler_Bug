 rust
#![feature(process)]
use std::process::Command;

fn main() {
    let res = Command::new("bogus").spawn();
    match res {
        Ok(_) => println!("ok"),
        Err(err) => println!("Error: {}", err),
    }
}
