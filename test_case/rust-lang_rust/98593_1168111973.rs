rust
let addr: *mut u8 = 0x12340000 as *mut u8; // pick an arbitrary page-aligned address
let addr = mmap(addr as *mut c_void, 0x10000, PROT_READ | PROT_WRITE, MAP_FIXED | MAP_PRIVATE, fd, 0);
