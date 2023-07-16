rust
use std::ops::*;
use std::marker::PhantomData;

// Define type-level integers
struct L0 {}
struct Succ<L> { _prev : L }
type L1 = Succ<L0>;
type L2 = Succ<L1>;

// Our nested data structure that includes the nesting level.
struct A<T,L> {
    v : T,
    lev : PhantomData<L>
}

// Nested multiplication trait. Default implementation is standard multiplication.
trait NestedMul<T, L> : Mul<T>  + Sized {
    fn nested_mul(self, a : T) -> Self::Output { self * a }
}

// Implement it for f64 using defaults.
impl<'b,T> NestedMul<T, L0> for f64 where f64 : Mul<T> { }

// Special implementation of NestedMul for A, bypassing Mul
impl<'b,L,T> NestedMul<&'b A<T,Succ<L>>,Succ<L>> for f64 where f64 : NestedMul<&'b T, L> {
    fn nested_mul(self, a : &'b A<T,Succ<L>>) -> Self::Output {
        A { v : self.nested_mul(&(a.v)), lev : PhantomData  }
    }
}

// The “interface” : when A is multiplied in plain code, we pass to level-counting nested
// multiplication to avoid compiler overflow. 
impl<'b, T, L> Mul<&'b A<T, Succ<L>>> for f64 where f64 : NestedMul<&'b T, L> { 
    type Output=A<<f64 as Mul<&'b T>>::Output, Succ<L>>;
    fn mul(self, a : &'b A<T,Succ<L>>) -> Self::Output {
        A { v : self.nested_mul(&(a.v)), lev : PhantomData }
    }
}

fn main() {
    let t : A<A<f64,L1>,L2> = A{ v : A{v : 1.0, lev : PhantomData }, lev : PhantomData };
    let _b = 3.0*&t;
}
