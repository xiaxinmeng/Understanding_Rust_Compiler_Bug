rust
let v = Vec::<u8>::with_capacity(100);
let mut sl = v.as_uninit();
if 0 != some_ffi_func(sl.first_ptr_mut(), sl.len()) {
    return Err<()>;
} else {
    v.set_len(sl.len());
    return Ok(v);
}
