rust
use core::{arch::x86_64::*, mem};
pub unsafe fn foo() {
    let x: __m128 = mem::zeroed();
    let x = _mm_cmpgt_ss(x, x);
    let x = _mm_cmpge_ss(x, x);
}
