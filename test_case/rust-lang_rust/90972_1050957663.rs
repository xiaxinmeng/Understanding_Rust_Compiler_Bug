Rust
use std::arch::aarch64::*;

#[target_feature(enable = "neon")]
unsafe fn aarch64_intrinsic() {
    let a = unsafe { vdupq_n_u8(7) };
    println!("a = {:?}", a);
}

pub fn main() {
        unsafe { aarch64_intrinsic() };
}
