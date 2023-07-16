rust
/// Returns the layout of a repr(C) struct with the given field layouts in the given order,
/// and the individual field's offsets.
pub fn repr_c(fields: &[Layout]) -> (Layout, Vec<usize>) {
    let mut offsets = Vec::new();
    let mut layout = Layout::from_size_align(0, 0).unwrap();
    for field in fields {
        let (new_layout, offset) = layout.extend(*field).unwrap();
        offsets.push(offset);
        layout = new_layout;
    }
    // Don't forget trailing padding!
    (layout.pad_to_align().unwrap(), offsets)
}
