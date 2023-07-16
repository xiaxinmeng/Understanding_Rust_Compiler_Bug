rust
pub fn repr_c<const N: usize>(fields: [Layout; N]) -> Result<(Layout, [usize; N]), LayoutErr> {
    let mut offsets: [usize; N] = unsafe { mem::zeroed() }; // replacement for `[0; N]` which doesn't work yet
    let mut layout = fields[0];
    for i in 1..N {
        let (new_layout, this_offset) = layout.extend(fields[i])?;
        layout = new_layout;
        offsets[i] = this_offset;
    }
    Ok((layout.pad_to_align()?, offsets))
}
