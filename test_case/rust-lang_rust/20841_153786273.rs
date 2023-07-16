 rust
fn do_async<F>(_cb: Box<F>) where F: FnOnce(&i32) {
}

fn do_async_unboxed<F>(cb: F) where F: FnOnce(&i32) {
    do_async(Box::new(cb))
}

fn main() {
    do_async_unboxed(|x| { println!("{}", *x); });   // Ok
    do_async(Box::new(|x| { println!("{}", *x); }));  // ERROR the type of this value must be known in this context
}
