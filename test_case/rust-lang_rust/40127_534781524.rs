
error[E0507]: cannot move out of `*x` which is behind a shared reference
 --> src/main.rs:5:21
  |
5 |         .filter(|x| x.unwrap().path().is_dir())
  |                     ^
  |                     |
  |                     move occurs because `*x` has type `std::result::Result<std::fs::DirEntry, std::io::Error>`, which does not implement the `Copy` trait
  |                     help: consider borrowing the `Result`'s content: `x.as_ref()`
