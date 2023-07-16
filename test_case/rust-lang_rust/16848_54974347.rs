 rust
#![allow(dead_code)]
trait A {}
fn foo<'a>() -> A + 'a { unimplemented!() }

fn main() {}
