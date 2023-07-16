
error: implementation of `Lambda` is not general enough
  --> src/lib.rs:18:5
   |
18 |     test(Compose(f, std::mem::drop as fn(T)));
   |     ^^^^
   |
   = note: Due to a where-clause on `test`,
   = note: `Compose<fn(&str) -> T, fn(T)>` must implement `Lambda<&'0 str>`, for any lifetime `'0`
   = note: but `Compose<fn(&str) -> T, fn(T)>` actually implements `Lambda<&'1 str>`, for some specific lifetime `'1`
