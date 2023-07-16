
➜  rust git:(macros/opaque-and-if-else) ✗ ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc test.rs --edition 2021
error[E0308]: `if` and `else` have incompatible types
  --> test.rs:10:9
   |
7  |       let _ = if true {
   |  _____________-
8  | |         async_dummy()
   | |         ------------- expected because of this
9  | |     } else {
10 | |         async_dummy2()
   | |         ^^^^^^^^^^^^^^ expected future, found a different future
11 | |     };
   | |_____- `if` and `else` have incompatible types
   |
   = note: distinct uses of `impl Trait` result in different opaque types
help: consider `await`ing on both `Future`s
   |
8  ~         async_dummy().await
9  |     } else {
10 ~         async_dummy2().await
   |

error: aborting due to previous error

