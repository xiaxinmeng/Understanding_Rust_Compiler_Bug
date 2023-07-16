rust
use std::ops::Neg;

pub struct Foo<T>(pub T);

impl<T: Neg<Output = T>> Neg for Foo<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Foo(-self.0)
    }
}

impl<T: Neg<Output = T> + Clone> Neg for &Foo<T> {
    type Output = Foo<T>;
    fn neg(self) -> Self::Output {
        Foo(-self.0.clone())
    }
}

// pub fn test0<T: Neg>(f: Foo<T>) -> Foo<T>{
//     f.neg()
// }
// pub fn test1<T: Neg>(f: Foo<T>) -> Foo<T>{
//     -f
// }
pub fn test2<T: Neg<Output = T>>(f: Foo<T>) -> Foo<T>{
    -f
}
// pub fn test3<T: Neg<Output = T>>(f: &Foo<T>) -> Foo<T>{
//     f.neg()
// }
pub fn test4<T: Neg<Output = T> + Clone>(f: &Foo<T>) -> Foo<T>{
    f.neg()
}
