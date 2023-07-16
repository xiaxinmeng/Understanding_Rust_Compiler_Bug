rust
use std::marker::PhantomData;

trait Lt<'a> {
    type T;
}
impl<'a> Lt<'a> for () {
    type T = PhantomData<&'a ()>;
}

fn test<'a>() {
    let _:fn(<() as Lt<'_>>::T) = |_:PhantomData<&'_ ()>| {};
}

fn main() {
    test();
}
