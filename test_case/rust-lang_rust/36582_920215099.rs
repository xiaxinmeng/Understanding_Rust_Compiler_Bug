
error: implementation of `FnOnce` is not general enough
  --> src/main.rs:11:5
   |
11 |     has_hrl(id);
   |     ^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 ()) -> &'2 () {id::<&'2 ()>}` must implement `FnOnce<(&'1 (),)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 (),)>`, for some specific lifetime `'2`
