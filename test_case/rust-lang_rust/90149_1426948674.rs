
error: implementation of `Send` is not general enough
  --> src/lib.rs:23:5
   |
23 |     Box::new(async move { async {}.with_props(vec!["foo"]).await })
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Send` is not general enough
   |
   = note: `Send` would have to be implemented for the type `&'0 str`, for any lifetime `'0`...
   = note: ...but `Send` is actually implemented for the type `&'1 str`, for some specific lifetime `'1`
   