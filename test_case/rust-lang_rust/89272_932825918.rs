rust
#![no_core]
#![feature(no_core)]

pub struct A;
pub type B = A;

pub trait T1{}
pub trait T2{}

impl T1 for A{}
impl T2 for B{}
