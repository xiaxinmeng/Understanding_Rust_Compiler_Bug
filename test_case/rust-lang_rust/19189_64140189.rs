 rust
pub fn align_of<T>() -> uint {
    // We use the preferred alignment as the default alignment for a type. This
    // appears to be what clang migrated towards as well:
    //
    // http://lists.cs.uiuc.edu/pipermail/cfe-commits/Week-of-Mon-20110725/044411.html
    unsafe { intrinsics::pref_align_of::<T>() }
}
