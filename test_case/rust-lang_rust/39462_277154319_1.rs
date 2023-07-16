rust
#[repr(C)] struct A(PhantomData<i32>); // reports warning here
#[repr(C)] struct B(A); // and also here if checks are recursive
