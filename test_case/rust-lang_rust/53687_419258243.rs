rust
use std::marker::PhantomData;

pub trait Unsigned {}

pub trait PrivatePow<Y, N> {
    type Output;
}

pub type PrivatePowOut<A, Y, N> = <A as PrivatePow<Y, N>>::Output;

pub trait Pow<N> {
    type Output;

    fn powi(self, x: N) -> Self::Output;
}

pub struct UTerm;
pub struct B1;
pub struct UInt<U, B> {
    _marker: PhantomData<(U, B)>,
}

pub type U1 = UInt<UTerm, B1>;

impl<X: Unsigned, N: Unsigned> Pow<N> for X
where
    X: PrivatePow<U1, N>,
{
        type Output = PrivatePowOut<X, U1, N>;
        fn powi(self, _: N) -> Self::Output {
            unsafe { ::core::mem::uninitialized() }
        }
}
