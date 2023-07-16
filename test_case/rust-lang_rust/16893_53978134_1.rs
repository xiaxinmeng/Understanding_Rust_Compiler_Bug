rust
#![feature(phase)]

#[phase(plugin, link)] extern crate foo;
fn main() {
    // example here
}
