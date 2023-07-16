rust
use std::ops::*;

// Our nested data structure
struct A<T> {
    v : T
}

// Nesting level counting structs and traits
struct L0 {}
struct Succ<L> { _prev : L }
trait Nested { type Level; }

// Primitives like f64 are at nesting level zero
impl Nested for f64 { type Level = L0; }
// A<T> always increases nesting level
impl<T> Nested for A<T> where T : Nested { type Level=Succ<T::Level>; }

// Nested multiplication trait. Default implementation is standard multiplication.
trait NestedMul<T, L> : Mul<T>  + Sized {
    fn nested_mul(self, a : T) -> Self::Output { self * a}
}
// Auto-implement it at level 0
impl<'b,S,T> NestedMul<&'b T,L0> for S where T : Nested<Level=L0>, S : Mul<&'b T> + Sized {}

// Special implementation of NestedMul for A, bypassing Mul
impl<'b, T : Nested> NestedMul<&'b A<T>, Succ<T::Level>> for f64 where f64 : NestedMul<&'b T, T::Level> {
    fn nested_mul(self, a : &'b A<T>) -> Self::Output {  A { v : self.nested_mul(&(a.v))  } }
}

// The “interface” : when A is multiplied in plain code, we pass to level-counting nested
// multiplication to avoid compiler overflow. 
//
// Version 1: this would be for all nesting structures. Not allowed by Rust as it involves
// no local structs. Similarly f64 has to be hard-coded here / this whole impl macro-generated
// when generalising to other types.
//
//impl<'b, T : Nested> Mul<&'b T> for f64 where f64 : NestedMul<T, T::Level>  { 
//    type Output=<f64 as Mul<&'b T>>::Output;
//    fn mul(self, a : &'b A<T>) -> Self::Output { self.nested_mul(a) }
//}
//
// Version 2: specifically for A<T>. A minor optimisation as bypasses one level of
// nested_mul:
impl<'b, T : Nested> Mul<&'b A<T>> for f64 where f64 : NestedMul<&'b T, T::Level> { 
   type Output=A<<f64 as Mul<&'b T>>::Output>;
   fn mul(self, a : &'b A<T>) -> Self::Output {  A { v : self.nested_mul(&(a.v)) } }
 }

fn main() {
    let t : A<A<f64>> = A{ v : A{v : 1.0 } };
    let _b = 3.0*&t;
}

