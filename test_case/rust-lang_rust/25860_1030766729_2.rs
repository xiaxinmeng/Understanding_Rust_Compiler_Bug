rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]
// nightly only
use std::marker::PhantomData;

struct Foo<T>(PhantomData<T>);

impl<'a, 'c, T: 'c> FnOnce<(&'a &'c (), &'c T)> for Foo<T> {
    type Output = ();
    extern "rust-call" 
    fn call_once(self, _: (&'a &'c (), &'c T)) -> Self::Output { todo!() }
}

// this compiles
fn bad1<'c, T: 'c>() {
    let x: &dyn for<'a> FnOnce(&'a &'c (), &'c T) = &Foo::<T>(PhantomData);
}

// this doesn't
// fn bad2<'c, T: 'c>() {
//    let x: &dyn for<'a> FnOnce(&'a &'static (), &'c T) = &Foo::<T>(PhantomData);
// }
