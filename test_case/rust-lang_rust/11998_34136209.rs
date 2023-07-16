 rust
static EMPTY: () = ();
static EMPTY_PTR: *mut u8 = &EMPTY as *() as *mut u8;

fn alloc(size: uint) -> *mut u8 {
    if size == 0 {
        EMPTY_PTR
    } else {
        ...
    }
}

fn free(ptr: *mut u8, size: uint) {
    if size != 0 {
        libc::free(ptr);
    }
}
