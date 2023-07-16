 rust
#![feature(phase)]
#[phase(syntax)] extern crate regex_macros;
extern crate regex;

fn main() {
    let regex = regex!(r"^\* (?P<branch>.*)\s*$");
}
