
use std::run::{Process, ProcessOptions};

fn main() {
    println!("{}", Process::new("echo", [~"foo"], ProcessOptions::new()).finish());
}
