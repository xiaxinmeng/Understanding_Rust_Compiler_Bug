rust
macro_rules! from_sse_to_avx { ($x:expr) => (merge_avx(split_sse($x)) }
macro_rules! from_avx_to_sse { ($x:expr) => (merge_sse(split_avx($x)) }
macro_rules! from_sse_to_avx_and_back { 
    ($f:expr, $x:expr) => (from_avx_to_sse!($f(from_sse_to_avx!($x)))) 
}
