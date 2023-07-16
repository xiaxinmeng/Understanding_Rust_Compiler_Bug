
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
  --> src/main.rs:22:5
   |
22 |     foo::<Option<Cow<'_,str>>, _>(f(|s| Cow::Owned(s.get(0..1).map(Cow::Borrowed))));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `FnOnce<(&String,)>`
              found trait `for<'r> FnOnce<(&'r String,)>`
note: this closure does not fulfill the lifetime requirements
  --> src/main.rs:22:37
   |
22 |     foo::<Option<Cow<'_,str>>, _>(f(|s| Cow::Owned(s.get(0..1).map(Cow::Borrowed))));
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
