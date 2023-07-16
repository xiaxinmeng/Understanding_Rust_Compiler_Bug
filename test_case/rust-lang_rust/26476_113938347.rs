 rust
use std::ops::Sub;

#[derive(Debug)]
struct Foo(u8);
#[derive(Debug)]
struct Bar(Foo, Foo);

impl<'a, 'b> Sub<&'a Foo> for &'b Foo {
    type Output = Foo;
    fn sub(self, lhs: &'a Foo) -> Foo {
        Foo(self.0 - lhs.0)
    }
}

impl<'a, 'b> Sub<&'a Bar> for &'b Bar {
    type Output = Bar;
    fn sub(self, lhs: &'a Bar) -> Bar {
        Bar(&self.0 - &lhs.0, &self.1 - &lhs.1)
    }
}

fn main() {
    let a = Bar(Foo(10), Foo(20));
    let b = Bar(Foo(5), Foo(7));
    println!("{:?}", &a - &b);
}
