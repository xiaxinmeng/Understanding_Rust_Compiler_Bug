
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
  --> src/main.rs:22:5
   |
22 |     Action::from(|_: &_| {});
   |     ^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> FnOnce<(&'a &'b (),)>`
              found trait `for<'a> FnOnce<(&'a &(),)>`
note: this closure does not fulfill the lifetime requirements
  --> src/main.rs:22:18
   |
22 |     Action::from(|_: &_| {});
   |                  ^^^^^^^

error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
  --> src/main.rs:22:5
   |
22 |     Action::from(|_: &_| {});
   |     ^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> Fn<(&'a &'b (),)>`
              found trait `for<'a> Fn<(&'a &(),)>`
note: this closure does not fulfill the lifetime requirements
  --> src/main.rs:22:18
   |
22 |     Action::from(|_: &_| {});
   |                  ^^^^^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to 2 previous errors
