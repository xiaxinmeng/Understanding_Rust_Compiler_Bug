rust
fn is_pow10_u64(v: u64) -> bool {
    let mut hash: u32 = v as u32;
    hash ^= hash >> 3;
    hash = (hash >> 24) ^ (hash >> 1);
    v == POWER10_HASH_U64[(hash & 31) as usize]
}
