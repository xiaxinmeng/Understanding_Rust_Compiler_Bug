rust
#![feature(never_type)]

#[derive(Debug)]
pub struct Foo(!);

#[derive(Debug)]
pub struct Bar(Empty);

#[derive(Debug)]
pub enum Empty {}
