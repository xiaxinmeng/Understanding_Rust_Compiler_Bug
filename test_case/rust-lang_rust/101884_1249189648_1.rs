
Compiling playground v0.0.1 (/playground)
error[[E0401]](https://doc.rust-lang.org/nightly/error-index.html#E0401): can't use generic parameters from outer function
 --> src/lib.rs:4:29
  |
3 | fn foo<T>() {
  |        - type parameter from outer function
4 |     const _: () = generic::<T>();
  |                             ^ use of generic parameter from outer function

For more information about this error, try `rustc --explain E0401`.
error: could not compile `playground` due to previous error
