
use std::util;

struct Foo;

impl Foo {
    fn eat(~self) { util::ignore(self); }
}

fn main() {
    (~Foo).eat();
}
