rust
#![no_core]
#![feature(no_core)]

pub struct A;

pub trait T{}

impl T for A{}

pub type B = A;
