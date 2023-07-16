rust
fn f() {
    let mut x = core::cell::RefCell::new(());
    let r = x.get_mut() as &();
    let _ = x.borrow_mut();
    drop(r);
}
