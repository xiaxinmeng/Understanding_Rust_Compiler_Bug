rust
trait X {
    type T;
}

trait Y<U>: X {
    fn foo(x: &Self::T);
}

impl X for () {
    type T = ();
}

impl<T> Y<Vec<T>> for () where (): Y<T> {
    fn foo(_x: &()) {}
}
