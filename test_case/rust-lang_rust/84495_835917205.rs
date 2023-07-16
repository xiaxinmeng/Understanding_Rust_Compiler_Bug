rust
trait Trait {
    // similar to Iterator::next
    fn f1(&self);
    // similar to Iterator::count
    fn f2(self) where Self: Sized {}
    // similar to Iterator::size_hint
    fn f3(&self) {}
}
// similar to impl<T: Iterator + ?Sized> Iterator for &mut T
impl<T: Trait> Trait for &T
{
    fn f1(&self) { (**self).f1() }
}
fn main() {
    1i32.f1();
    1i32.f2();
    1i32.f3();
}
