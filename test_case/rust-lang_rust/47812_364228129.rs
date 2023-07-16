
lunch-box. rustc issue-47812.rs -L .
error: proc-macro derive panicked
 --> issue-47812.rs:4:10
  |
4 | #[derive(Panic)]
  |          ^^^^^
  |
  = help: message: no can do
