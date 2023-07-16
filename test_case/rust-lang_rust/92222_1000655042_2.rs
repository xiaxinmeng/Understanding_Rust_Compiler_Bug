
         .           fn layout_to_flags(align: usize, size: usize) -> c_int {
99,245,696 ( 1.97%)      if align <= ALIGNOF_MAX_ALIGN_T && align <= size {
         .                   0
         .               } else {
         .                   ffi::MALLOCX_ALIGN(align)
         .               }
 9,022,336 ( 0.18%)  }
