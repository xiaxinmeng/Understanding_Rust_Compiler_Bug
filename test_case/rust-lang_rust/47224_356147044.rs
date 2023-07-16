rust
fn f(foo: RefCell<Foo>) {
    let mut foo = &mut *foo.borrow_mut();
    
    foo.v.push(foo.b);
}
