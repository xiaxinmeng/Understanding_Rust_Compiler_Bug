rust
use std::ops::*;

struct A<T>  {
    v : T
}

impl<T> Mul<f64> for A<T> where T : Mul<f64> {
    type Output = A<<T as Mul<f64>>::Output>;
    fn mul(self, w : f64) -> Self::Output { A{ v : <T as Mul<f64>>::mul(self.v, w) } }
}

impl<T> Mul<A<T>> for f64 where f64 : Mul<T> {
    type Output = A<<f64 as Mul<T>>::Output>;
    fn mul(self, x : A<T>) -> Self::Output { A{ v : <f64 as Mul<T>>::mul(self, x.v) } }
}

impl<'a, T> Mul<f64> for &'a A<T> where &'a T : Mul<f64> {
    type Output = A<<&'a T as Mul<f64>>::Output>;
    fn mul(self, w : f64) -> Self::Output { A{ v : <&'a T as Mul<f64>>::mul(&(self.v), w)} }
}

impl<'b, T> Mul<&'b A<T>> for f64 where f64 : Mul<&'b T> { 
    type Output = A<<f64 as Mul<&'b T>>::Output>;
    fn mul(self, x : &'b A<T>) -> Self::Output { A { v : <f64 as Mul<&'b T>>::mul(self, &(x.v)) } }
}

fn main() {
    let t = A{ v : A{v : 1.0 } };
    let _a = <f64 as Mul<&A<A<f64>>>>::mul(3.0, &t); // This explicit typing works
    //let _b = 3.0*(&t); //  If you uncomment this, the compiler overflows.
    let _c = &t*3.0;
}
