
fn bar(mut buckets: &mut [u32]) {
    let key = 0u32;
    buckets[{buckets = &mut []; (key as usize).wrapping_add(22).wrapping_rem(buckets.len())}] = 22;
}
