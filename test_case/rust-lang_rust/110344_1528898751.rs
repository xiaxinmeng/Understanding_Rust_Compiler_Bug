rust
alignment_enum! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(usize)]
    enum AlignmentEnum {
        if #[cfg(target_pointer_width = "16")] {
            ...
        }
        if #[cfg(target_pointer_width = "32")] {
            ...
        }
        if #[cfg(target_pointer_width = "64")] {
            ...
        }
    }
}
