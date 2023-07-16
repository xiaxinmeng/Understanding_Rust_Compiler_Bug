rust
trait A {
    type B;
}
trait M {}

struct G<T, U>(*const T, *const U);

impl<T, U> Clone for G<T, U> {
    fn clone(&self) -> Self {
        G { ..*self }
    }
}

impl<T, U> Copy for G<T, U::B>
where 
T: A<B = U>,
U: A
{}

impl A for () {
    type B = ();
}

fn is_m<T: M>(_: T) {}

fn main() {
    let x = G(&(), &());
    drop(x);
    drop(x);
}
