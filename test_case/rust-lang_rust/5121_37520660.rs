 rust
trait MyTrait<'a> {
    fn yo(&'a self);
}

struct MyStruct<'a> {
    inner: &'a  MyTrait<'a>,
}

impl<'a> MyStruct<'a> {

    fn foo_inner(&mut self) {
        self.inner.yo()
    }
}
