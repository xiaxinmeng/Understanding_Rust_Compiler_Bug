 rust
#[repr(C)] struct T(Vec<u8>, Vec<u32>, PhantomData<X>);
#[repr(C)] struct U(Vec<u8>, Vec<u32>, PhantomData<Y>);
