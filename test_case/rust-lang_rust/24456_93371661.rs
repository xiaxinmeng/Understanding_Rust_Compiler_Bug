 Rust
fn forget<T>(val: T) {
    use std::cell::RefCell;
    use std::rc::Rc;
    struct Foo<T>(T, RefCell<Option<Rc<Foo<T>>>>);
    let x = Rc::new(Foo(val, RefCell::new(None)));
    *x.1.borrow_mut() = Some(x.clone());
}
struct DontDropMe;
impl Drop for DontDropMe {
    fn drop(&mut self) { unreachable!() }
}
fn main() {
    forget(DontDropMe)
}
