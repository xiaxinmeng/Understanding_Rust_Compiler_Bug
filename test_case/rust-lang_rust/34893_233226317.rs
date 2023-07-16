 Rust
#![feature(unboxed_closures)]
pub trait FnXXX<Args> {
    type Output;
    fn z(&self, x: Self::Output) -> Self::Output;
    fn zzz() -> Self::Output;
}
impl<A,F:?Sized> FnXXX<A> for F
    where F : Fn<A>
{
    type Output = F::Output;
    fn z(&self, x: Self::Output) -> Self::Output { x }
    fn zzz() -> Self::Output { unimplemented!() }
}
fn f<T>(t: T)
    where T : for<'a> Fn(&'a i64) -> &'a i64
{
    let y = 10;
    let mut x = &y;
    x = <T as FnXXX<(&i64,)>>::z(&t, x);
    let a: &'static i64 = <T as FnXXX<(&'static i64,)>>::zzz();
}
fn main() {
  f(|x| x);
}
