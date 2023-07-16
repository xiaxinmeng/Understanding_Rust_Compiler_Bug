
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:8:46
  |
6 |     constrain::<i32, _>(s);
  |                         - here the type of `s` is inferred to be `Wrap<Option<i32>, Option<_>>`
7 |     constrain::<_, i32>(s);
8 |     let z: Wrap<Option<i32>, Option<&str>> = s;
  |            -------------------------------   ^ expected `&str`, found `i32`
  |            |
  |            expected due to this
  |
  = note: expected struct `Wrap<_, Option<&str>>`
             found struct `Wrap<_, Option<i32>>`

For more information about this error, try `rustc --explain E0308`.
