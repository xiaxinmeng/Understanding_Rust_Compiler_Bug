 rust
trait Y { fn f(&self) { } }

struct X;

impl Y for X { }

struct Z<Trait: ?Sized> {
    _data: std::marker::PhantomData<Trait>,
}

fn f<Y: ?Sized>(y: &Y) -> Z<Y> {
    Z { _data: std::marker::PhantomData }
}

fn main() {
    {
        let arc = std::sync::Arc::new(X);
        f(&*arc as &(Y+'static))
    };
}
