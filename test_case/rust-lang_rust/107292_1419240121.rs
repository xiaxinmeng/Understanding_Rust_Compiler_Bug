
error[E0282]: type annotations needed
 --> src/main.rs:4:28
  |
4 |     println!("{}", arr[idx.into()]);
  |                            ^^^^
  |
help: try using a fully qualified path to specify the expected types
  |
4 |     println!("{}", arr[<i32 as Into<T>>::into(idx)]);
  |                        +++++++++++++++++++++++   ~

