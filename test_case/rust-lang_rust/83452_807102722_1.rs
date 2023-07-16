rust
struct X<T>(T);

impl<T> X<T> {
    const GET_X: usize = <T as Trait>::FOO;
}

pub const fn get_usize(self) -> usize {
    X::<T>::GET_X
}
