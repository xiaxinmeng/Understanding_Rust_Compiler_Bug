plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
  --> library/alloc/src/tests.rs:28:26
   |
28 |             assert!(a == 8);
   |                          ^ expected struct `std::boxed::Box`, found integer
   = note: expected struct `std::boxed::Box<i32>`
                found type `{integer}`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
28 |             assert!(a == Box::new(8));
   |                          +++++++++ +
error[E0308]: mismatched types
  --> library/alloc/src/tests.rs:34:26
   |
   |
34 |             assert!(a == Test);
   |                          ^^^^ expected struct `std::boxed::Box`, found struct `Test`
   = note: expected struct `std::boxed::Box<Test>`
              found struct `Test`
              found struct `Test`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
34 |             assert!(a == Box::new(Test));
   |                          +++++++++    +
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:02:09
