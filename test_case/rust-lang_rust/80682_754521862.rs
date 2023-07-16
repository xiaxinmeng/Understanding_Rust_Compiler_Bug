rust
fn swap<T>(slice: &mut [T], a: usize, b: usize) {
    let slice = Cell::from_mut(slice).as_slice_of_cells();
    slice[a].swap(&slice[b]);
}
