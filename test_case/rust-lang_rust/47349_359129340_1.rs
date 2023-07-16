
fn bar(buckets: &mut [u32]) {
    let key = 0u32;
    buckets[(key as usize).wrapping_add(22).wrapping_rem(buckets.len())] = 22;
}
