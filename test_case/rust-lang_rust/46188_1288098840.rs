rust
#[no_mangle]
pub unsafe fn a() -> *const u8 {
    extern { pub fn malloc(_: usize) -> &'static (); }
    malloc(usize::MAX) as *const _ as *const _
}
#[no_mangle]
pub unsafe fn b() -> *const u8 {
    extern { pub fn malloc(_: usize) -> *const u8; }
    let p = malloc(usize::MAX);
    if p.is_null() { panic!(); }
    p
}
