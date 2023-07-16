
3 |     Some(true).filter(|| {
  |                ^^^^^^ -- takes 0 arguments
  |                |
  |                expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
3 |     Some(true).filter(|_| {
  |                       ~~~
