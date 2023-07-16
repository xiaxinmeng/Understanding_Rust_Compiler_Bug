
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
 --> src/main.rs:6:18
  |
6 |     takes_option(&res);
  |     ------------ ^^^^
  |     |            |
  |     |            expected enum `Option`, found `&Option<String>`
  |     |            help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`: `&res.as_ref()`
  |     arguments to this function are incorrect
  |
  = note:   expected enum `Option<&String>`
          found reference `&Option<String>`
note: function defined here
 --> src/main.rs:1:4
  |
1 | fn takes_option(_arg: Option<&String>) {}
  |    ^^^^^^^^^^^^ ---------------------

For more information about this error, try `rustc --explain E0308`.

