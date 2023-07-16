rust
use std::ops::Add;

struct Foo;

impl<'a> Add<&'a Foo> for &'a Foo {
    type Output = Foo;
    fn add(self, rhs: &'a Foo) -> Self::Output { Foo }
}

fn foo<'a, 'b>(x: &'a Foo, y: &'b Foo) -> Foo {
    x + y
 // ^^^^^ cannot infer an appropriate lifetime for lifetime parameter `'a`
 //       due to conflicting requirements
}

fn main() { }
