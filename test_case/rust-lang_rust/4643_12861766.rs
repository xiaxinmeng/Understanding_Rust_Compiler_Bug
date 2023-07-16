
trait A { fn f(); }

impl P: A { fn f() {} }
impl Q: A { #[deprecated] fn f() {} }

fn g<T: A>(x: T) {
    x.f();
}
