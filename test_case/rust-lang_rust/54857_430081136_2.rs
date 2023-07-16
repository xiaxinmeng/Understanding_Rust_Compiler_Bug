rust
>   use std::slice;
>
>   fn make_slice(arr: &[u8; 5]) -> &[u8] {
>       let integer = arr.as_ptr() as usize;
>       // At index 0.
>       let ptr = integer as *const u8;
>       // Move to index 10 (out of bounds). If this pointer moves back in bounds, it
>       // can be dereferenced. Note that if `.offset()` was used instead, it would
>       // be undefined behavior to dereference the pointer (and all pointers
>       // produced by offsetting it) due to the constraints on `.offset()`. As a
>       // result, if we used `.offset()` here, we would not be able to safely create
>       // the slice (because a non-zero-length slice can dereference its pointer).
>       let ptr = ptr.wrapping_offset(10);
>       // Move to index 1 (in bounds again).
>       let ptr = ptr.wrapping_offset(-9);
>       // Move to index 2 (still in bounds).
>       let ptr = unsafe { ptr.offset(1) };
>       // Slice corresponds to indices 2..4 (all in bounds).
>       unsafe { slice::from_raw_parts(ptr, 2) }
>   }
>   