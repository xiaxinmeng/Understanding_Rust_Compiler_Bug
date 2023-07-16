rust
#[no_mangle]
#[cfg(overflow_checks)]
pub fn cast(v: i64)->u32{
    v.try_into().unwrap()
}

#[no_mangle]
#[cfg(not(overflow_checks))]
pub fn cast(v: i64)->u32{
    v as _
}
