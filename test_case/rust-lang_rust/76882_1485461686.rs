rust
#![feature(type_alias_impl_trait)]
#![feature(unsize)]

use core::marker::Unsize;
use core::fmt::Debug;

pub fn returns_unrelated_impl<T>() -> impl Debug + 'static { () }

// doesn't compile: the compiler thinks the `impl Debug` should capture `'a`
pub fn test_1<'a>() -> impl Debug {
    returns_unrelated_impl::<&'a ()>()
}

// doesn't compile: the compiler thinks `ImplUnsizeStatic` should capture `'a`
pub fn test_2a<'a>() -> impl Debug {
    type ImplUnsizeStatic = impl Unsize<dyn Debug> + 'static;

    let rv1: ImplUnsizeStatic = returns_unrelated_impl::<&'a ()>();
    let rv2: Box<dyn Debug> = Box::new(rv1);
    rv2
}

// does compile, because we aren't assigning an `impl` to another `impl`
pub fn test_2b<'a>() -> impl Debug {
    fn verify_impl_unsize_static<T: Unsize<dyn Debug> + 'static>(x: &T) {}

    let rv1 = returns_unrelated_impl::<&'a ()>();
    verify_impl_unsize_static(&rv1);
    let rv2: Box<dyn Debug> = Box::new(rv1);
    rv2
}
