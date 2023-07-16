rust
#[repr(packed)]
#[derive(Eq, PartialEq)]
pub struct FlexZeroSlice {
    // Hard Invariant: 1 <= width <= USIZE_WIDTH (which is target_pointer_width)
    // Soft Invariant: width == the width of the largest element
    width: u8,
    // Hard Invariant: data.len() % width == 0
    data: [u8],
}
