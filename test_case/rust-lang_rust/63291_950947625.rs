rust
pub struct Allocation {
    ptr: NonNull<u8>,
    layout: Layout
}

// Some methods to convert Vec/Box/etc -> Allocation and back (if the layout matches)
