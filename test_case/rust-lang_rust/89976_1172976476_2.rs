rust
error: implementation of `Iterator` is not general enough
  --> src/lib.rs:44:5
   |
44 |     assert_send(broken1(&[]));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Iterator` is not general enough
   |
   = note: `Iterator` would have to be implemented for the type `std::slice::Iter<'0, String>`, for any lifetime `'0`...
   = note: ...but `Iterator` is actually implemented for the type `std::slice::Iter<'1, String>`, for some specific lifetime `'1`

error: implementation of `FnOnce` is not general enough
  --> src/lib.rs:44:5
   |
44 |     assert_send(broken1(&[]));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'0 String) -> std::future::Ready<()>` must implement `FnOnce<(&String,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(&String,)>`
