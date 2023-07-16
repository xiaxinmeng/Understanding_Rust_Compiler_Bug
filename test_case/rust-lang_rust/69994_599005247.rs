rust
#![feature(unboxed_closures, fn_traits)]

use std::marker::PhantomData;

trait Trait<'a> { }
impl<'a, T> Trait<'a> for T { }

struct Func<'a, T>(PhantomData<&'a T>);
const CONST: &dyn FnOnce(&str) = &Func::<u32>(PhantomData);

impl<'a, T: Trait<'a>> FnOnce<(&'a str,)> for Func<'a, T> {
    type Output = ();    
    extern "rust-call" fn call_once(self, _: (&'a str,)) { }
}
