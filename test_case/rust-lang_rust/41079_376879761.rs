rust
pub fn offset_to(self, other: *const T) -> Option<isize> where T: Sized {…}
pub unsafe fn offset_from(self, origin: *const T) -> isize where T: Sized {…}
pub fn wrapping_offset_from(self, origin: *const T) -> isize where T: Sized {…}
