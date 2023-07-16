
% cat src/main.rs
#![feature(plugin)]
#![plugin(regex_macros)]

extern crate regex;

static R: regex::Regex = regex!(".+");

fn main() {
    println!("Hello, world!");
}
