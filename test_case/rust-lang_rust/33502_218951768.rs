
use std::marker::PhantomData;
struct Zero;
struct Succ<T>(PhantomData<T>);
trait Nat {
    fn slow<T>();
}
impl Nat for Zero {
    fn slow<T>() {}
}
impl<N: Nat> Nat for Succ<N> {
    fn slow<T>() {
        N::slow::<(f32,T)>();
        N::slow::<(f64,T)>();
    }
}
fn slow<N: Nat>() {
    N::slow::<()>();
}
fn explode<N: Nat>(){
    slow::<N>();
    explode::<Succ<N>>();
}
fn main(){
    explode::<Zero>();
}
