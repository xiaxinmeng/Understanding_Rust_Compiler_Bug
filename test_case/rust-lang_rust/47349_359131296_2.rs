
let index = {
    buckets = &mut [];
    (key as usize).wrapping_add(22).wrapping_rem(buckets.len())
};
if index < buckets.len() {
    buckets[index] = 22;
}
