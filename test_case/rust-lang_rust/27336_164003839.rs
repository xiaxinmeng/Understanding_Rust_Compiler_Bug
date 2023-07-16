 rust
#![crate_type = "lib"]
#![feature(default_type_parameter_fallback)]

trait A<T = Self> {
    fn a(t: &T) -> Self;
}

trait B<T = Self> {
    fn b(&self) -> T;
}

impl<U, T = U> B<T> for U
    where T: A<U>
{
    fn b(&self) -> T {
        T::a(self)
    }
}

struct X(u8);

impl A for X {
    fn a(x: &X) -> X {
        X(x.0)
    }
}

fn f(x: &X) {
    x.b(); // ok
}

fn g(x: &X) {
    let x = x.b();
    x.0; // error: the type of this value must be known in this context
}
