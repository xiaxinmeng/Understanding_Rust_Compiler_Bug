 Rust

#[no_mangle]
pub extern "C"  fn test(s: S) -> u32 {
    s.c
}
