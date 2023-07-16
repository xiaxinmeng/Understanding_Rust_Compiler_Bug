rust
use std::marker::PhantomData;

struct Foo<T>(PhantomData<T>);
trait MyFnOnce<Args> {}
impl<'a, 'c, T: 'c> MyFnOnce<(&'a &'c (), &'c T)> for Foo<T> {}

// this compiles
fn good<'c, T: 'c>() {
    let foo2: &dyn for<'a> MyFnOnce<(&'a &'c (), &'c T)> = &Foo::<T>(PhantomData);
}

// this doesn't compile
// fn bad<'c, T: 'c>() {
//    let foo3: &dyn for<'a> MyFnOnce<(&'a &'static (), &'c T)> = &Foo::<T>(PhantomData);
// }
