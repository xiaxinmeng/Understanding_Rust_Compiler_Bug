plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: call to unsafe function is unsafe and requires unsafe block (error E0133)
  --> library/alloc/src/collections/vec_deque/iter.rs:35:9
   |
35 |         core::slice::from_raw_parts(self.ring.as_ptr() as *const MaybeUninit<T>, self.ring.len())
   |
note: the lint level is defined here
  --> library/alloc/src/lib.rs:82:9
   |
   |
82 | #![deny(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> library/alloc/src/collections/vec_deque/iter.rs:47:53
   |
47 |         let (front, back) = RingSlices::ring_slices(self.ring(), self.head, self.tail);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> library/alloc/src/collections/vec_deque/iter.rs:90:53
   |
90 |         let (front, back) = RingSlices::ring_slices(self.ring(), self.head, self.tail);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/alloc/src/collections/vec_deque/iter.rs:115:33
    |
115 |             let (front, back) = self.ring().split_at(self.tail);
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/alloc/src/collections/vec_deque/iter.rs:172:53
    |
172 |         let (front, back) = RingSlices::ring_slices(self.ring(), self.head, self.tail);
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/alloc/src/collections/vec_deque/iter.rs:197:33
    |
197 |             let (front, back) = self.ring().split_at(self.tail);
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
