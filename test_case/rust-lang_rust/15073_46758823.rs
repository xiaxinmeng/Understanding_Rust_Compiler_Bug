
struct Foo<'a> {
    bar: &'a mut int
}

trait Trait {
    fn baz(&mut self);
}

impl<'a> Foo<'a> {
    fn get<'a>(&'a mut self) -> &'a mut int {
        return &mut *self.bar
    }
}

impl<'a> Trait for Foo<'a> {
    fn baz(&mut self) {
        let a = self.get();
    }
}
