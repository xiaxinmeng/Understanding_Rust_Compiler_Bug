rust
#![feature(never_type)]
fn main() {
    let x: !;
    let c = || match x { }; // ERROR: borrow of possibly-uninitialized variable: `x`
}
