rust
#![feature(const_generics)]
fn bad() where for<'b> [();{let _:&'b (); 0}]: Sized { }
fn good() where for<'b> [();{0}]: Sized { }
