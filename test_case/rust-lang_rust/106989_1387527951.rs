rust
fn is_zero<T: StructuralEq>(x: &T) -> bool {
    match try_const { unsafe { core::mem::MaybeUninit::<T>::zeroed().assume_init() } } {
        Some(zero) => *x == zero,
        None => false,
    }
}
