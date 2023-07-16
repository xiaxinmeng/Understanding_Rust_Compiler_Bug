rust
use std::collections::BTreeSet;

fn uniq_refs<'i, 'd: 'i, T>(
    data: &'d mut [T],
    indices: &'i BTreeSet<usize>,
) -> impl Iterator<Item = &'d mut T> + 'i {
    let in_bounds_indices = indices.range(0..data.len());

    in_bounds_indices.map(move |&i| {
        // I copied this from a Stack Overflow answer 
        // without reading the text that explains why this is safe
        unsafe {
            let r = data.get_unchecked_mut(i);
            let p: *mut T = r;
            &mut *p
        }
    })
}
