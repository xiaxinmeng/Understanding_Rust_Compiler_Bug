rust
use std::marker::PhantomData;
struct A<T> {
    x: Option<Box<A<T>>>,
    _z: PhantomData<T>,
}

fn main() {
    let a = A::<i32> {
        x: None,
        _z: PhantomData
    };
    assert!(a.x.is_none());
}
