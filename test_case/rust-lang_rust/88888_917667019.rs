rust
use std::arch::x86_64::*;

trait GenU8 {
    const X: i32;
}

unsafe fn f<X: GenU8>(v: __m128i) -> __m128i {
    _mm_shufflelo_epi16(v, X::X)
}
