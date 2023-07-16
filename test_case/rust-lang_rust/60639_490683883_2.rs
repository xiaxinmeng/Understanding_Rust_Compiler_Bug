rust
fn uniq_refs<'i, 'd: 'i, T>(
    data: &'d mut [T],
    indices: &'i BTreeSet<usize>,
) -> impl Iterator<Item = &'d mut T> + 'i {
    let start = data.as_mut_ptr();
    let in_bounds_indices = indices.range(0..data.len());
    in_bounds_indices.map(move |&i| unsafe { &mut *start.add(i) })
}
