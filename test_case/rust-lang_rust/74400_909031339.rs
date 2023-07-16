
error: implementation of `FnOnce` is not general enough
  --> src/lib.rs:11:5
   |
11 |     is_sorted_and_has_duplicates_by(data, identity)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 T) -> &'2 T {identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
