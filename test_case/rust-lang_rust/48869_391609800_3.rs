rust
trait Foo {
    fn foo(&self) -> u32;
}

impl<T: Foo> Foo for Box<T> {
    fn foo(&self) -> u32 {
        self.as_ref().foo()
    }
}

trait Bar {
    fn bar(&self) -> u32;
}

impl<T: Foo> Bar for Vec<T> {
    fn bar(&self) -> u32 {
        self.iter().map(|x| x.foo()).sum()
    }
}
