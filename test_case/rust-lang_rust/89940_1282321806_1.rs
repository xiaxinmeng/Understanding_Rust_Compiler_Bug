
Compiling playground v0.0.1 (/playground)
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the trait bound `[Element]: ToOwned` is not satisfied in `Element`
 --> src/lib.rs:5:25
  |
5 |     First( Cow<'static, [Element]>)
  |                         ^^^^^^^^^ within `Element`, the trait `ToOwned` is not implemented for `[Element]`
  |
  = help: the trait `ToOwned` is implemented for `[T]`
note: required because it appears within the type `Element`
 --> src/lib.rs:4:6
  |
4 | enum Element {
  |      ^^^^^^^
  = note: slice and array elements must have `Sized` type

error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the trait bound `[Element]: ToOwned` is not satisfied in `Element`
 --> src/lib.rs:3:10
  |
3 | #[derive(Clone)]
  |          ^^^^^ within `Element`, the trait `ToOwned` is not implemented for `[Element]`
  |
  = help: the trait `ToOwned` is implemented for `[T]`
note: required because it appears within the type `Element`
 --> src/lib.rs:4:6
  |
4 | enum Element {
  |      ^^^^^^^
note: required by a bound in `Clone`
  = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to 2 previous errors
