rust
use std::marker::PhantomData;

struct B0;
struct B1;
struct UTerm;
struct UInt<T, U> { _marker: PhantomData<(T, U)>, }
struct Tensor<T, U> { x: (T, U) }
struct Test2;

trait Mul<T> { type Output; }
trait Variance { type Rank; }
trait PrivatePow<T, U> { type Output; }
trait CoordinateSystem { type Dimension; }

impl<T, U> Mul<UInt<T,U>> for UTerm {
    type Output = UTerm;
}
impl<T, U, V> Mul<UInt<V, U>> for UInt<T, B0> where T: Mul<UInt<V, U>> {
    type Output = UInt<<T as Mul<UInt<V, U>>>::Output, B0>;
}
impl<T, U, V> Mul<UInt<V, U>> for UInt<T, B1> where T: Mul<UInt<V, U>> {
    type Output = UInt<V, U>;
}
impl<T, U, V, W> PrivatePow<T, UInt<U, V>> for W
    where W: Mul<W> + Mul<T>,
         <W as Mul<W>>::Output: PrivatePow<<W as Mul<T>>::Output, UInt<U, V>>
{
    type Output = <<W as Mul<W>>::Output as PrivatePow<<W as Mul<T>>::Output, UInt<U, V>>>::Output;
}
impl<T, U> Tensor<T, U>
    where T: CoordinateSystem,
          U: Variance,
          T::Dimension: PrivatePow<UInt<UTerm, B1>, U::Rank> {
    fn new() {}
}
impl CoordinateSystem for Test2 {
    type Dimension = UInt<UInt<UTerm, B1>, B0>;
}

fn main() {
    Tensor::<Test2, ()>::new
}
