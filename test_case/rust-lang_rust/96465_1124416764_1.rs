
error[[E0311]](https://doc.rust-lang.org/nightly/error-index.html#E0311): the parameter type `T` may not live long enough
  [--> src/lib.rs:29:14
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=f4b62bb6c9773f5807cd8fe25ae8266a#)   |
29 |         self.function(|value| {})
   |              ^^^^^^^^ ...so that the type `Struct<T>` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound...
   |
19 | impl<T: 'a> Trait for Struct<T> {
   |       ++++
