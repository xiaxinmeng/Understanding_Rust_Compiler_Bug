rust
fn foo<T>() {
    // can't use generic parameters from outer function:
    // const ASSERT: () = assert!(mem::size_of::<T>() == 8);
    let _ = Assert::<T>::SIZE_EQ_8;
}

struct Assert<T> {
    _marker: PhantomData<T>,
}

impl<T> Assert<T> {
    const SIZE_EQ_8: () = assert!(mem::size_of::<T>() == 8);
}
