rust
let x = Box::into_raw(Box::new(0u32));
let x = x.wrapping_offset(8); // okay, this has no inbounds tag
let x = unsafe { x.offset(0) }; // UB, the pointer is not inbounds of the only object it can point to
