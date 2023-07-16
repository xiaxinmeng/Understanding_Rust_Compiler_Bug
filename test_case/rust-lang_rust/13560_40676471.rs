 rust
#![feature(phase)]

#[phase(syntax)]
extern crate fourcc;
#[phase(syntax, link)]
extern crate log;
