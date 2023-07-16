rust
/// Slice from `vec[vec.len()..vec.capacity()]`
pub unsafe fn remaining_capacity_as_slice_mut<A>(vec: &mut Vec<A>) -> &mut [A] {
    let range = vec.len()..vec.capacity();
    vec.get_unchecked_mut(range)
}
