
#![feature(phase)]

#[phase(plugin)]
extern crate green;

green_start!(main)

fn main() {
    std::task::spawn(proc() {
        println!("hello world");
    });
}
