rust
/// An internal function for calculating pointer offsets as usizes, while accounting
/// directly for possible ZSTs. This is used specifically in the iterator implementations.
#[inline(always)]
pub(crate) const fn distance_between<T>(dest: *const T, origin: *const T) -> usize {
  match size_of::<T>() {
    0 => (dest as usize).wrapping_sub(origin as usize),
    // Safety: this function is used strictly with linear inputs
    // where dest is known to come after origin.
    _ => unsafe { ptr_offset_from(dest, origin) as usize },
  }
}
