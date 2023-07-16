rust
trait A {
    fn f1<'a>(&'a mut self) {}
}

impl<T> A for T {}

unsafe fn f3<'a>(x: *mut ()) {
    (&x, (*x).f1());
}
