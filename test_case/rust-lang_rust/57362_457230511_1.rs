
// error: implementation of `Trait` is not general enough
//   --> src/main.rs:13:7
//    |
// 13 |     a.f();
//    |       ^
//    |
//    = note: `Trait` would have to be implemented for the type `fn(&'0 u8)`, for some specific lifetime `'0`
//    = note: but `Trait` is actually implemented for the type `for<'r> fn(&'r u8)`
