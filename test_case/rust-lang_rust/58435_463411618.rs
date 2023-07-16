rust
struct _S<T>(T);

impl<T> _S<T> {
    const _ID: fn(_S<T>) = |_| {};

    pub fn _id(self) {
        _S::<T>::_ID(self)
    }
}

fn _foo() {
    _S(())._id();
}
