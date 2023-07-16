rust
>   use std::slice;
>
>   fn make_slice(arr: &[u8; 5]) -> &[u8] {
>       let integer = arr.as_ptr() as usize;
>       // At index 0.
>       let ptr = integer as *const u8;
>       // Move to index 10 (out of bounds). This pointer (and all pointers
>       // produced by offsetting it) must never be dereferenced due to the
>       // constraints on `.offset()`.
>       let ptr = unsafe { ptr.offset(10) };
>       // This is okay because a zero-length slice will never dereference its
>       // pointer.
>       unsafe { slice::from_raw_parts(ptr, 0) }
>   }
>   