
#![feature(const_raw_ptr_to_usize_cast)]

const A: usize = &42 as *const i32 as usize;
