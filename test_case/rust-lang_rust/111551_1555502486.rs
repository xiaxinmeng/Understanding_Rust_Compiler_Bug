rs
#[repr(C)]
#[repr(align(16))]
pub struct TwoU64s {
    pub a: u64,
    pub b: u64,
}

extern "C" {
    fn many_args(h: TwoU64s) -> i32;
}
