 rust
fn main() {}

trait MyAdd<RHS,Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

impl<'self, T: Copy> MyAdd<&'self [T],~[T]> for ~[T] {
    fn add(&self, rhs: & &'self [T]) -> ~[T] { rhs.to_owned() }
}

impl<'self, T: Copy> MyAdd<&'self [T],~[T]> for ~[T] {
    fn add(&self, rhs: & &'self [T]) -> ~[T] { rhs.to_owned() }
}
