rust
fn flat_map(query: &mut [&mut [f32]]) {
    query.iter_mut().flat_map(AsMut::as_mut).for_each(|value| *value += 1.0);
}
