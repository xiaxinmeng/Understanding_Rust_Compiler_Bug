rust
fn is_pow10_u64(v: u64) -> bool {
    let mut hash: u32 = v as u32;
    hash ^= hash >> 3;
    hash = hash.rotate_right(1);
    hash ^= hash >> 23;
    v == POWER10_HASH_U64[(hash & 31) as usize]
}
