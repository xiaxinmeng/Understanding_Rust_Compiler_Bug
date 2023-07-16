rust
let vec = unsafe { slice::from_raw_parts(param2.val as *const u8, param2.val_len as usize) };
