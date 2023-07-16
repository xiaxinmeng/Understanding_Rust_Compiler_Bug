
// src/main.rs
#![feature(phase)]

#[phase(plugin)] extern crate compile_msg;

fn main() {
    compile_note!("hello");
}
