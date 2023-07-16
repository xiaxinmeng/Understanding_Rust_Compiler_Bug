
fn forget<T>(val: T) {
    use std::cell::RefCell;
    use std::rc::Rc;
    struct Foo<T>(T, RefCell<Option<Rc<Foo<T>>>>);
    let x = Rc::new(Foo(val, RefCell::new(None)));
    *x.1.borrow_mut() = Some(x.clone());
}
fn main() {
    {
        let mut a = 10;
        forget(&mut a); // Leak the reference to `a`
        let b = &mut a; // Now we have two mut references to `a`, one here and one somewhere
    }
    // The dangling reference to `a` still lives somewhere. Can it be made accessible without `unsafe`?
}
