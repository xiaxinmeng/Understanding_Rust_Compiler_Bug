rust
use std::ops::*;

struct A<T> {
    v : T
}

impl<T> Mul<f64> for A<T> where T : Mul<f64> {
    type Output = A<<T as Mul<f64>>::Output>;
    fn mul(self, w : f64) -> Self::Output { A{ v : self.v * w} }
}

impl<T> Mul<A<T>> for f64 where f64 : Mul<T> {
    type Output = A<<f64 as Mul<T>>::Output>;
    fn mul(self, x : A<T>) -> Self::Output { A{ v : self * x.v} }
}

impl<'a, T> Mul<f64> for &'a A<T> where &'a T : Mul<f64> {
    type Output = A<<&'a T as Mul<f64>>::Output>;
    fn mul(self, w : f64) -> Self::Output { A{ v : &(self.v) * w} }
}

// If you remove this
impl<'b, T> Mul<&'b A<T>> for f64 where f64 : Mul<&'b T> {
    type Output = A<<f64 as Mul<&'b T>>::Output>;
    fn mul(self, x : &'b A<T>) -> Self::Output { A { v : self * &(x.v) } }
}

fn main() {
    let t = A{v : 1.0};
    let _b = 3.0*&t; // ... and this, then it compiles.
    let _c = &t*3.0;
}
