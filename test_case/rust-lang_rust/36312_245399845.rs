 rust
union Inner<T: ?Sized> {
    value: T,
}

pub struct NoDrop<T: ?Sized>(Inner<T>);
