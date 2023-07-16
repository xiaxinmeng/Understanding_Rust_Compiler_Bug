
error[[E0282]](https://doc.rust-lang.org/stable/error-index.html#E0282): type annotations needed
 --> src/main.rs:4:28
  |
4 |     let val: u32 = arr[idx.into()];
  |                            ^^^^
  |
help: try using a fully qualified path to specify the expected types
  |
4 |     let val: u32 = arr[<u8 as Into<T>>::into(idx)];
  |                        ++++++++++++++++++++++   ~

For more information about this error, try `rustc --explain E0282`.
