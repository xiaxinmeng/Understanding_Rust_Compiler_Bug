
tmp0 = SliceWrapper::slice_mut(&mut buckets); // starts the borrow of `buckets`
let index = (key as usize).wrapping_add(22).wrapping_rem(buckets.sweep());
if index < tmp0.len() {
    tmp0[index] = 22;
}
