rust
let layout = Layout::from_size_align(isize::MAX as usize + 1, 1).unwrap();
unsafe {
    let ptr = alloc(layout);
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    dealloc(ptr, layout);
}
println!(
    "successfully made 2^{} byte allocation",
    layout.size().trailing_zeros()
);
assert_eq!(layout.size().count_ones(), 1);
assert_eq!(layout.size().leading_zeros(), 0);
