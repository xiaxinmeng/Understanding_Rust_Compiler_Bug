 rust
struct Wrapper<T: ?Sized>(T);

impl<T: ?Sized> Drop for Wrapper<T> {
    fn drop(&mut self) {}
}

fn main() {
    let wrapper = Box::new(Wrapper(123));
    let _: Box<Wrapper<Send>> = wrapper;
}
