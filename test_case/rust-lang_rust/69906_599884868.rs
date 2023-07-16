rust
#![feature(decl_macro)]
macro x() { struct X; }

x!();
x!();
