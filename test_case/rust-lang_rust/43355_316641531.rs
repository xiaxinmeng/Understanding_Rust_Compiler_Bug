rust
trait Trait1 {
    type Output;
}
fn f<T: Trait1>() {
    ::std::mem::size_of::<T::Output>();
}
fn main() {}
