
  |
3 | async fn return_number() -> usize {
  | ----- by annotating this function with `async` makes it `fn() -> impl Future<Output = usize>` instead of `fn() -> usize`
...
