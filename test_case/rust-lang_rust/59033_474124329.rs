rust
use std::marker::PhantomData;

pub struct SomeStruct<'a, T: 'a> {
    _marker: PhantomData<&'a T>,
}
