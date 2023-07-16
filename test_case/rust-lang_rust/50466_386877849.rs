
   Compiling alloc v0.0.0 (file:///home/nikic/rust/src/liballoc)
error[E0152]: duplicate lang item found: `box_free`.
   --> liballoc/alloc.rs:158:1
    |
158 | / unsafe fn old_box_free<T: ?Sized>(ptr: *mut T) {
159 | |     box_free(Unique::new_unchecked(ptr))
160 | | }
    | |_^
    |
    = note: first defined in crate `alloc`.

error: aborting due to previous error
