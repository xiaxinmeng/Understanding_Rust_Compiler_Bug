rust
#![feature(step_trait)]
use std::iter::Step;
use std::ops::RangeFrom;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Foo(u8);

impl Step for Foo {
    // dummies
    fn steps_between(_: &Foo, _: &Foo) -> Option<usize> { unimplemented!() }
    fn replace_one(&mut self) -> Foo { unimplemented!() }
    fn replace_zero(&mut self) -> Foo { unimplemented!() }
    fn sub_one(&self) -> Foo { unimplemented!() }
    fn add_usize(&self, _: usize) -> Option<Foo> { unimplemented!() }
    
    // required for this example
    fn add_one(&self) -> Foo {
        if self.0 >= 10 {
            Foo(0)
        } else {
            Foo(self.0 + 1)
        }
    }
}

fn main() {
    let range = RangeFrom { start: Foo(0) };
    for i in range.take(20) {
        println!("{:?}", i);
    }
}
