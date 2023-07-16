
// error: implementation of `Trait` is not general enough
//   --> src/main.rs:13:7
//    |
// 13 |     a.f();
//    |       ^
//    |
//    = note: `fn(&u8)` must implement `Trait`
//    = note: but `for<'r> fn(&'r _)` only implements `Trait`
