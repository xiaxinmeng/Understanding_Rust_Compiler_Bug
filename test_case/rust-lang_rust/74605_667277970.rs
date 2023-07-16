rust
let (slice, cap) = vec.leak();

let vec = unsafe { Vec::from_raw_parts(slice.as_mut_ptr(), slice.len(), cap) };
