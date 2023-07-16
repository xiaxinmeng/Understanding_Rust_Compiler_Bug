rust
trait Foo {
    fn foo(&self) -> u32;
}

trait Bar {
    fn bar(&self) -> u32;
}

impl<T: Foo> Bar for Vec<T> {
    fn bar(&self) -> u32 {
        self.iter().map(|x| x.foo()).sum()
    }
}

impl<T: Foo> Bar for Vec<Box<T>> {
    fn bar(&self) -> u32 {
        self.iter().map(|x| x.foo()).sum()
    }
}
