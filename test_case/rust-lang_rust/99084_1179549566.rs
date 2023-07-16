rust
let mut value: u8 = 0;
let ptr: *mut bool = &mut value as *mut u8 as *mut bool;
ptr.write_bytes(42, 1);
