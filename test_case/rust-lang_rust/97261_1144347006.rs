rust
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
pub struct NonZeroUsize(usize);
