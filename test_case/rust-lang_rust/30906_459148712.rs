
error: implementation of `std::ops::FnOnce` is not general enough
  --> src/lib.rs:20:5
   |
20 |     test(Compose(f, |_| {}));
   |     ^^^^
   |
   = note: Due to a where-clause on `test`,
   = note: `Compose<fn(&str) -> T, [closure@src/lib.rs:20:21: 20:27]>` must implement `std::ops::FnOnce<(&'0 str,)>`, for any lifetime `'0`
   = note: but `Compose<fn(&str) -> T, [closure@src/lib.rs:20:21: 20:27]>` actually implements `std::ops::FnOnce<(&'1 str,)>`, for some specific lifetime `'1`
