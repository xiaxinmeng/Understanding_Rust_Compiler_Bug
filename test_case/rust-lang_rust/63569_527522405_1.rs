rust
let v_uninit = Vec::<MaybeUninit<u8>>::with_capacity(1024);
let result = some_ffi_func(v_uninit.as_mut_ptr(), v_uninit.capacity());
match result {
    Ok(len) => Ok(v_uninit.assume_init(len)),
    Err(e) => Err(e)
}
