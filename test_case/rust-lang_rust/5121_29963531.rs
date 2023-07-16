
trait Iterable<'a> {
    fn my_iter(&'a self);
}

struct Foo;

impl <'a> Foo {
    fn foo<C: Iterable<'a>>(c: &'a C) {
        c.my_iter();
    }
}

pub fn main() { }
