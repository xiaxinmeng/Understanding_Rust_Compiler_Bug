 rust
#![feature(phase, macro_rules)]
#[phase(plugin)]
extern crate "span-lint" as _lint;

macro_rules! bar {
    ($e: expr) => { $e }
}

macro_rules! foo {
    ($e: expr) => { bar!($e) };
    () => { 1u }
}

fn main() {
    foo!(1u);
    foo!();
}
