
Compiling playground v0.0.1 (/playground)
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the trait bound `&'static mut (): Clone` is not satisfied
 [--> src/lib.rs:3:5
](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=eba7813b633302c60221d58389a58468#)  |
3 |     &'static mut (): Clone,
  |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&'static mut ()`
  |
  = help: see issue #48214

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
