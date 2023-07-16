rust

// In-place rotation algorithm (shifts to the right)
fn rotate_slice<T>(slice: &mut [T], places: usize) {
    // Rotation can be implemented by reversing the slice,
    // splitting the slice in two, and then reversing the
    // two sub-slices.
    slice.reverse();
    let (a, b) = slice.split_at_mut(places);
    a.reverse();
    b.reverse();
}
