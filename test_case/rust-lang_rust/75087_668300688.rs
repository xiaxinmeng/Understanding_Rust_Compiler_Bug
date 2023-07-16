rust
trait MapVecInPlace<T> {
    fn map_in_place<U>(self, f: impl FnMut(T) -> U) -> Vec<U>;
}

impl<T> MapVecInPlace<T> for Vec<T> {
    fn map_in_place<U>(mut self, mut f: impl FnMut(T) -> U) -> Vec<U> {
        assert_eq!(size_of::<T>(), size_of::<U>());
        assert_eq!(align_of::<T>(), align_of::<U>());

        let ptr_range_start = self.as_mut_ptr();
        // Safety: Both pointers are of the same type T, and no wrapping is involved.
        let ptr_range_end = unsafe { ptr_range_start.add(self.len()) };

        let mut ptr = ptr_range_start;
        while ptr != ptr_range_end {
            // Safety: The value is within the range of pointers where memory is initialized.
            let t_value = unsafe { ptr.read() };
            let u_value = f(t_value);
            // Safety: We have a `U` returned from `f`. As we're writing this value, we need to
            // cast this pointer. This is safe because size and alignment have already been
            // checked.
            unsafe { ptr.cast::<U>().write(u_value) };

            // Safety: No wrapping is involved, as we're operating within the original vector.
            ptr = unsafe { ptr.add(1) };
        }

        // Safety: We've transformed all T's to U's. Size and alignment is guaranteed to be the
        // same due to asserts.
        unsafe { transmute(self) }
    }
}
