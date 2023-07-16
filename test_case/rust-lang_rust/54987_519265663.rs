rust
fn construct_pair_inplace<T, U>(
    out: &mut MaybeUninit<(T, U)>,
    mk0: impl FnOnce() -> T,
    mk1: impl FnOnce() -> U,
) {
    let ptr = out.as_mut_ptr();
    ptr::write(&raw mut (*ptr).0, mk0());
    ptr::write(&raw mut (*ptr).1, mk1());
}
