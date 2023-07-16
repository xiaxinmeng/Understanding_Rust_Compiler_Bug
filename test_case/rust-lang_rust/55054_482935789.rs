rust
pub unsafe fn fn_cttz(x: u64) -> u32 {
    if x == 0 { 0 } else { cttz(x) as u32 }
}
