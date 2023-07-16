rust
#![feature(generic_const_exprs)]

unsafe fn zeroed<T: Sized>() -> T 
where
    [(); std::mem::size_of::<T>()]:
{
    // First create an array that has the same size as T, initialized at 0
    let x = [0u8; std::mem::size_of::<T>()];
    // Then, transmute it to type T, and return it
    std::mem::transmute_copy(&x)
}
