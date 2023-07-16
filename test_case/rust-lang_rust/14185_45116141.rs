 rust
#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

static FOO: regex::Regex = regex!("foo");

fn main() {
}
