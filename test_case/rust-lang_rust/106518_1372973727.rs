rust
#[rustfmt::skip]
fn works() {}

#[::rustfmt::skip]
fn doesnt() {}

use rustfmt as x;
#[x::skip]
fn also_doesnt() {}
