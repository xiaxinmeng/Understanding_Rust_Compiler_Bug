rust
>   use std::slice;
>
>   fn make_slice(arr: &[u8; 5]) -> &[u8] {
>       // At index 0.
>       let ptr = arr.as_ptr();
>       // Move to index 10 (out of bounds). If this pointer moves back in bounds, it
>       // can be dereferenced. Note that this would be undefined behavior if
>       // `.offset()` was used instead.
>       let ptr = ptr.wrapping_offset(10);
>       // Move to index 1 (in bounds again).
>       let ptr = ptr.wrapping_offset(-9);
>       // Move to index 2 (still in bounds).
>       let ptr = unsafe { ptr.offset(1) };
>       // Slice corresponds to indices 2..4 (all in bounds).
>       unsafe { slice::from_raw_parts(ptr, 2) }
>   }
>   