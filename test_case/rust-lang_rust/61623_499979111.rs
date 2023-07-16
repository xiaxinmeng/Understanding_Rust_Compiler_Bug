rust
fn f1<'a>(_: &'a mut ()) {}

unsafe fn f3<'a>(x: *mut ((), &'a mut ())) {
    (&x, f1((*x).1));
}
