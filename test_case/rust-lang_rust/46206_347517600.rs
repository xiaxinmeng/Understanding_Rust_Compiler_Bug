rust
#![feature(default_type_parameter_fallback)]
#![feature(specialization)]

use std::fmt::Debug;

trait B { fn b(&self) -> Self; }

impl<T=String> B for Option<T> where T: Default
{
    default fn b(&self) -> Option<T> {
        Some(T::default())
    }
}
// When there specialized but generic impls, their defaults
// are ignored no matter what they are.
// This code does not compile because `x` in main fails to infer.
// However if we commented out this impl, `x` would be inferred to `String`.
impl<T=String> B for Option<T> where T: Default + Debug
{
    fn b(&self) -> Option<T> { Some(T::default()) }
}

fn main() {
    let x = None;
    let y = x.b();
}
