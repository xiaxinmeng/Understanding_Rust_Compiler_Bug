
let (ptr, len, cap) = vec_of_uninit_bytes.into_raw_parts();
let vec = Vec::<u8>::from_raw_parts(ptr as *mut u8, len, cap);
