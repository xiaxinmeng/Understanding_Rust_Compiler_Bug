
error[[E0401]](https://doc.rust-lang.org/nightly/error-index.html#E0401): can't use generic parameters from outer function
 --> src/main.rs:3:21
  |
2 | fn testing<T>() {
  |            - type parameter from outer function
3 |     const TESTING: [T; 0] = [];
  |                     ^ use of generic parameter from outer function

For more information about this error, try `rustc --explain E0401`.
