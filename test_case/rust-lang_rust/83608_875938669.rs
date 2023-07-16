
impl<'h,T> &'h mut [T] {
    /// Returns an initial segment of a `mut &'h mut [T]` replacing the inner
    /// `&'h mut [T]` with the remainder.  In effect, this executes the command
    /// `(ret,heap) = heap.split_at_mut(len)` without annoying the borrow
    /// checker.
    pub fn reserve_mut(&mut self, len: usize) -> &'h mut [T] {
        let tmp: &'h mut [T] = ::core::mem::replace(&mut *self, &mut []);
        let (reserved, tmp) = tmp.split_at_mut(len);
        *self = tmp;
        reserved
    }
}
