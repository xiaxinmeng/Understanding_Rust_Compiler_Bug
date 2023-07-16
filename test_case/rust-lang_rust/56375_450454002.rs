rust
use std as attr;

#[attr] //~ ERROR cannot determine resolution for the attribute macro `attr`
fn main() {}
