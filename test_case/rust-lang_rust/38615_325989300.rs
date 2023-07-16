rust
#![feature(generators, generator_trait, conservative_impl_trait)]
use std::ops::{Generator};

struct Stack { 
    x: i32
}

impl Stack {
    fn new(x: i32) -> Stack { Stack{x: x} }
    fn gen(& mut self) -> impl Generator<Yield=i32,Return=()> { 
 // fn gen<'a>(&'a mut self) -> impl Generator<Yield=i32,Return=()> + 'a { 
        move || { for _i in 0..10 { self.x = self.x + 1; yield self.x; } return (); } 
    }
}
