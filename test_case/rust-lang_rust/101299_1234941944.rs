plain
    Checking rustc-demangle v0.1.21
error: implementation has missing stability attribute
  --> library/alloc/src/collections/vec_deque/iter.rs:35:1
   |
35 | unsafe impl<T: Sync> Sync for Iter<'_, T> {}

error: implementation has missing stability attribute
  --> library/alloc/src/collections/vec_deque/iter.rs:36:1
   |
   |
36 | unsafe impl<T: Send> Send for Iter<'_, T> {}

error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:14
