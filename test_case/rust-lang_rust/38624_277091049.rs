Rust
pub trait BorrowMut<Borrowed> where Borrowed: ?Sized {
    fn borrow_mut(&mut self) -> &mut Borrowed;
}

impl<T> BorrowMut<T> for Box<T> where T: ?Sized {
    fn borrow_mut(&mut self) -> &mut T { panic!() }
}

#[cfg(not(works))]
impl<T> BorrowMut<T> for T where T: ?Sized {
    fn borrow_mut(&mut self) -> &mut T { panic!() }
}
trait T {}

impl T for u8 {}
fn main() {
    // D (BorrowMut and Trait Object):
    let mut r: Box<T> = Box::new(23u8);
    let _: &mut T = BorrowMut::borrow_mut(&mut r);
}
