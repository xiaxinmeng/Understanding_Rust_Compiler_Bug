 rust
#![feature(plugin)]
#![plugin(regex_macros)]

extern crate regex;

static R : regex::Regex = regex!(r".+");

fn main() {
    println!("Hello, world!");
}
