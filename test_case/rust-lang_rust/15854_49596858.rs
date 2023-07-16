 rust
#![feature(macro_rules)]

fn returns_unit() { }

macro_rules! function( () => { returns_unit() } )

fn main() {
    function!()
    function!()
    function!()
}
