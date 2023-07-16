 rust
struct Wrapper<'a, T: ?Sized>(&'a mut i32, T);
impl<'a, T: ?Sized> Drop for Wrapper<'a, T> {
    fn drop(&mut self) {}
}
fn main() {
    let mut x = 3;
    let wrapper: Box<Wrapper<i32>> = Box::new(Wrapper(&mut x, 123));
    assert_eq!(std::mem::size_of_val(&*wrapper), 16);
    let wrapper2: Box<Wrapper<Send>> = wrapper;
    assert_eq!(std::mem::size_of_val(&*wrapper2), 16); // currently fails
}
