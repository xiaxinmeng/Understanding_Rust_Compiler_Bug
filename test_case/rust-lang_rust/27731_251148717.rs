
fn simd_mul<T>(v1: T, v2: T) -> T
where std::mem::size_of<T>(): platform_simd_size, std::mem::align_of<T>(): platform_simd_align {
//magic code
}
