rust
trait A {
    type X;
}

trait B: A {
    type X; // note: this is legal
}

trait BP<P, Q>: A<X = P> + B<X = Q> {}

impl<T> Clone for Box<dyn BP<T, T>> {
    fn clone(&self) -> Self {
        todo!()
    }
}

fn main() {
    let v: Box<dyn BP<(), ()>>;
}

