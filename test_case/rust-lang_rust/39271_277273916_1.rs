rust
const F32_NAN: u32 = 0x7F800000u32;
const F32_NAN_MASK: u32 = 0x7FFFFFFFu32;
unsafe fn is_signal_nan(x: *const f32) -> bool {
        let ptr: *const u32 = mem::transmute(x);
        let mut val: u32 = read_volatile(ptr);
        (val & F32_NAN_MASK) == F32_NAN
}
const F64NAN: 7FF0000000000000u64;
const F64_NAN_MASK: 7FFFFFFFFFFFFFFFu64;
unsafe fn is_signal_nan(x: *const f64) -> bool {
        let ptr: *const u64 = mem::transmute(x);
        let mut val: u64 = read_volatile(ptr);
        (val & F64_NAN_MASK) == F64NAN
}
