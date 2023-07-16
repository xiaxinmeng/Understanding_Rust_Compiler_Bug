
209 // XXX: ptr::offset is positive ints only
210 #[inline]
211 pub fn mut_offset<T>(ptr: *mut T, count: int) -> *mut T {
212     use core::sys::size_of;
213     (ptr as int + count * (size_of::<T>() as int)) as *mut T
214 }
