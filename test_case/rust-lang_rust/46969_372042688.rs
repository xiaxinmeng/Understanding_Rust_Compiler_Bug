rust
use std::marker::PhantomData;

trait Const {
    type T;
    const VAL: Self::T;
}

struct MakeTuple<U>(PhantomData<U>);
impl<U: Const + Sized> Const for MakeTuple<U> {
    type T = (U::T, U::T);
    const VAL: Self::T = (U::VAL, U::VAL);
}
