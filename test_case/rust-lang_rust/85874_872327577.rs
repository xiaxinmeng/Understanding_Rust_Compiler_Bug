rust
impl<T, I> SpecInPlaceCollect<T, I> for I
where
    I: Iterator<Item = T>,
{
    #[inline]
    default fn collect_in_place(&mut self, dst_buf: *mut T, end: *const T) -> usize {
        // use try-fold since
        // - it vectorizes better for some iterator adapters
        // - unlike most internal iteration methods, it only takes a &mut self
        // - it lets us thread the write pointer through its innards and get it back in the end
        let sink = InPlaceDrop { inner: dst_buf, dst: dst_buf };
        let sink =
            self.try_fold::<_, _, Result<_, !>>(sink, write_in_place_with_drop(end)).unwrap();
        // iteration succeeded, don't drop head
        unsafe { ManuallyDrop::new(sink).dst.offset_from(dst_buf) as usize }
    }
}

impl<T, I> SpecInPlaceCollect<T, I> for I
where
    I: Iterator<Item = T> + TrustedRandomAccess,
{
    #[inline]
    fn collect_in_place(&mut self, dst_buf: *mut T, end: *const T) -> usize {
        let len = self.size();
        let mut drop_guard = InPlaceDrop { inner: dst_buf, dst: dst_buf };
        for i in 0..len {
            // Safety: InplaceIterable contract guarantees that for every element we read
            // one slot in the underlying storage will have been freed up and we can immediately
            // write back the result.
            unsafe {
                let dst = dst_buf.offset(i as isize);
                debug_assert!(dst as *const _ <= end, "InPlaceIterable contract violation");
                ptr::write(dst, self.__iterator_get_unchecked(i));
                // Since this executes user code which can panic we have to bump the pointer
                // after each step.
                drop_guard.dst = dst.add(1);
            }
        }
        mem::forget(drop_guard);
        len
    }
}
