rust


pub fn dynamic_fast(layout: Layout, size: usize) -> Result<Layout, LayoutErr>{
    let align = layout.align();
    if !align.is_power_of_two() {
        unsafe { core::hint::unreachable_unchecked(); }
    }
    Layout::from_size_align(size, align)
}
