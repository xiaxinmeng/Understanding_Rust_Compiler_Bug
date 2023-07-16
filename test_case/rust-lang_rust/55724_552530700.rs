rust
pub fn repr_c<const N: usize>(fields: [Layout; N]) -> Result<(Layout, [usize; N]), LayoutErr> {
    unsafe {
        let mut offsets: MaybeUninit<[usize; N]> = MaybeUninit::uninit();
        let mut layout = Layout::from_size_align_unchecked(0, 0);
        let mut offset = offsets.as_mut_ptr().cast::<usize>();
        for field in &fields[..] {
            let (new_layout, this_offset) = layout.extend(*field)?;
            layout = new_layout;
            ptr::write(offset, this_offset);
            offset = offset.offset(1);
        }
        Ok((layout.pad_to_align()?, offsets.assume_init()))
    }
}
