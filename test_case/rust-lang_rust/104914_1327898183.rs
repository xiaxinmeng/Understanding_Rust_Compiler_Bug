rust
#[no_mangle]
fn flat_map(query: &mut [&mut [f32]]) {
    let iter = query.iter_mut().flat_map(|segment| segment.iter_mut());
    for value in iter {
        *value += 1.0;
    }
}
