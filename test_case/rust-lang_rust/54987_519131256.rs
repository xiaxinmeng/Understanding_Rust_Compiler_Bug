rust
fn construct_pair_inplace(
    out: &mut MaybeUninit<(String, !)>,
    mk_string: impl FnOnce() -> String,
    mk_void: impl FnOnce() -> !,
) {
    let ptr = out.as_mut_ptr();
    ptr::write(&raw mut (*ptr).0, mk_string());
    ptr::write(&raw mut (*ptr).1, mk_void());
}
