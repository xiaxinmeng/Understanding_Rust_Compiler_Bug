rust
pub unsafe fn mir_transmute<T, U>(x: T) -> U {
    #[repr(C)]
    union Transmute<T, U> { t: ManuallyDrop<T>, u: ManuallyDrop<U> }
    ManuallyDrop::into_inner(Transmute { t: ManuallyDrop::new(x) }.u)
}
