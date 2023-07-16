
Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:6:23
  |
6 |     uwu(async move || {})
  |                       ^^ expected `Box<_>`, found `async` closure body
  |
  = note:            expected struct `Box<_>`
          found `async` closure body `[async closure body@src/main.rs:6:23: 6:25]`
  = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
  |
6 |     uwu(async move || Box::new(()))
  |                       ++++++++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to previous error
