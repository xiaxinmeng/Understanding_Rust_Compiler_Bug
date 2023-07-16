rust
Compiling playground v0.0.1 (/playground)
error[[E0412]](https://doc.rust-lang.org/nightly/error-index.html#E0412): cannot find type `T` in this scope
 --> src/main.rs:4:37
  |
4 |     println!("{}", arr[<i32 as Into<T>>::into(idx)]);
  |                                     ^ not found in this scope

For more information about this error, try `rustc --explain E0412`.
error: could not compile `playground` due to previous error
