rust
use std::marker::PhantomData;

struct UInt<T, U> { _marker: PhantomData<(T, U)>, }
struct Tensor<T, U> { x: (T, U) }

trait Mul<T> { type Output; }
trait Variance { type Rank; }
trait PrivatePow<T, U> { type Output; }
trait CoordinateSystem { type Dimension; }

impl<U, V> Mul<UInt<U, V>> for UInt<(), ()> {
    type Output = UInt<(), ()>;
}
impl<T, U, V, W> PrivatePow<T, UInt<U, V>> for W
    where W: PrivatePow<W, UInt<U, V>>
{
    type Output = UInt<(), ()>;
}
impl<T, U> Tensor<T, U>
    where T: CoordinateSystem,
          U: Variance,
          T::Dimension: PrivatePow<UInt<(), ()>, U::Rank> {
    fn new() {}
}
impl CoordinateSystem for () {
    type Dimension = UInt<(), ()>;
}

fn main() {
    Tensor::<(), ()>::new
}
