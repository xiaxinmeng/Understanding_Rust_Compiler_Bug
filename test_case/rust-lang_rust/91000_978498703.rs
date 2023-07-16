rust
// mod ptr;

/// # Safety
/// - that of pointer::offset
///   - pointers into same allocated object
///   - distance is a multiple of size_of::<T>() bytes
unsafe fn slice_from_ptr_range_mut<T>(range: Range<*mut T>) -> *mut [T] {
    slice_from_raw_parts_mut(range.start, range.start.offset(range.end))
}

fn slice_from_wrapping_ptr_range_mut(range: Range<*mut T> -> *mut [T] {
    slice_from_raw_parts_mut(range.start, range.start.wrapping_offset(range.end))
}

impl<T> *mut [T] {
    /// # Safety
    /// Valid pointer & length, no assertions of memory content's validity
    /// Key is that this can use inbounds pointer offsets, and
    /// slice references can use this and just `&mut*` the pointer
    unsafe fn iter(self) -> slice::RawIter {
        // it's slice::Iter but yielding raw pointers
    }
}
