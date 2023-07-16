rust
fn rot<T : std::ops::Shl<Output = T>>(x:&T, k:u32) -> T {
    let bit_count = std::mem::size_of::<T>() << 3;
    (x << k as T) | (x >> (bit_count - k as T))
}
