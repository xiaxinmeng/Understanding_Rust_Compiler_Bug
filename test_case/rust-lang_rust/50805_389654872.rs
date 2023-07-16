rust
trait Foo {
    fn foo(self);
}

struct Bar<'a>(&'a mut i32);

impl<'a> Foo for Bar<'a> {
    fn foo(mut self) {
        self.0 = 42;
    }
}

const fn foo<T: const Foo>(x: T) {
    x.foo(); // lint would presumably fire, but this does something
}
