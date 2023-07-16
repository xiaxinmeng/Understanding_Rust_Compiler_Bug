rust
let addr = ptr.addr();
let ptr = addr as *const u8;
let _val = *ptr;
