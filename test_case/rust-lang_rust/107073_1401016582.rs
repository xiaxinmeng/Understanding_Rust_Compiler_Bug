rust
#![feature(type_alias_impl_trait)]

type MyIter = impl Iterator<Item = i32>;

struct Foo {
    iter: MyIter,
}

impl Foo {
    fn set_iter(&mut self) {
        self.iter = [1, 2, 3].into_iter();
    }
}
