
error: implementation of `Lambda` is not general enough
  --> src/lib.rs:18:5
   |
18 |     test(Compose(f, std::mem::drop as fn(T)));
   |     ^^^^
   |
   = note: `Lambda<&'0 str>` would have to be implemented for the type `fn(&str) -> T`, for any lifetime `'0`
   = note: but `Lambda<&'1 str>` is actually implemented for the type `fn(&'1 str) -> T`, for some specific lifetime `'1`
