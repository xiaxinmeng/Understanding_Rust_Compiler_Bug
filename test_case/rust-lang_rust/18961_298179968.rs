rust
use std::marker::PhantomData;

struct Foo<'a>(PhantomData<&'a i32>);

impl<'a> Foo<'a> {
    fn qux(&'a mut self) {}
}

fn main() {
   let mut foo = Foo(PhantomData);
   foo.qux();
   foo.qux();
}
