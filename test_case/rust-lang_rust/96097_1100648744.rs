rust
unsafe trait Uninitialized {
    type Initialized: ?Sized;

    fn uninit() -> Self
    where Self: Sized;
    fn zeroed() -> Self
    where Self: Sized;
    
    unsafe fn assume_init(self) -> Self::Initialized
    where Self: Sized;
    unsafe fn assume_init_ref(&self) -> &Self::Initialized;

    ...
}

unsafe impl<T> Uninitialized for MaybeUninit {...}
unsafe impl<T, const N: usize> Uninitialized for [T; N] {...}
unsafe impl<T> Uninitialized for [T] {...}
