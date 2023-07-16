 rust
use std::marker::PhantomData;
use std::ops::{Add, Mul};
struct B0;
struct B1;
struct UTerm;
struct UInt<U, B> {
    _marker: PhantomData<(U, B)>,
}
impl Add for UTerm
{
    type Output = UTerm;
    fn add(self, _: UTerm) -> Self {
    }
}
impl<U, B> Add<UInt<U,B>> for UTerm {
    type Output = UInt<U, B>;
    fn add(self, _: UInt<U, B>) -> Self::Output {}
}

impl<Ul, Ur> Add<UInt<Ur, B0>> for UInt<Ul, B0>
    where Ul: Add<Ur>
{
    type Output = UInt<<Ul as Add<Ur>>::Output, B0>;
    fn add(self, _: UInt<Ur, B0>) -> Self::Output {}
}

impl<Ul, Ur> Add<UInt<Ur, B1>> for UInt<Ul, B0>
    where Ul: Add<Ur>
{
    type Output = UInt<<Ul as Add<Ur>>::Output, B1> ;
    fn add(self, _: UInt<Ur, B1>) -> Self::Output {
    }
}

trait Pow<Rhs> { type Output; }
impl<U,B> Mul<UInt<U,B>> for UTerm
{
    type Output = UTerm;
    fn mul(self, _: UInt<U, B>) -> Self {
    }
}

impl<Ul, B, Ur> Mul<UInt<Ur, B>> for UInt<Ul, B0>
    where Ul: Mul<UInt<Ur, B>>
{
    type Output = UInt<<Ul as Mul<UInt<Ur, B>>>::Output, B0>;
    fn mul(self, _: UInt<Ur, B>) -> Self::Output {}
}

impl<Ul, B, Ur> Mul<UInt<Ur, B>> for UInt<Ul, B1>
    where Ul: Mul<UInt<Ur, B>>, UInt<<Ul as Mul<UInt<Ur, B>>>::Output, B0>:Add<UInt<Ur,B>>
{
    type Output = <UInt<<Ul as Mul<UInt<Ur, B>>>::Output, B0> as Add<UInt<Ur, B>>>::Output;
    fn mul(self, _: UInt<Ur, B>) -> Self::Output {}
}

type U1 = UInt<UTerm, B1>;
type U2 = UInt<U1, B0>;
trait PrivatePow<Y, N> { type Output; }
impl<X,N> Pow<N> for X where X: PrivatePow<U1, N>
{
    type Output = <X as PrivatePow<U1, N>>::Output;
}

impl<Y, U, B, X> PrivatePow<Y, UInt<UInt<U, B>, B1>> for X
    where X: Mul + Mul<Y>,
         <X as Mul>::Output: PrivatePow<<X as Mul<Y>>::Output, UInt<U, B>>
{
    type Output = <<X as Mul>::Output as PrivatePow<<X as Mul<Y>>::Output, UInt<U, B>>>::Output;
}

trait Variance { type Rank; }

trait CoordinateSystem {
    type Dimension;
}

type Power<T, U> =<T as Pow<U>>::Output;
struct Tensor<T, U> {
    x: (T, U)
}
impl<T, U> Tensor<T, U>
    where T: CoordinateSystem,
          U: Variance,
          T::Dimension: Pow<U::Rank> {
    fn new() {}
}

type Vector<T> = Tensor<T, ()>;
struct Test2;
impl CoordinateSystem for Test2 {
    type Dimension = U2;
}

fn main() {
    Vector::<Test2>::new
}
