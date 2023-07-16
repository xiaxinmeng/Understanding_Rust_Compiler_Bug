
error[E0720]: recursion is not supported in async fn
 --> src/main.rs:3:24
  |
3 | async fn foo(n: usize) {
  |                        ^ async fn cannot invoke themselves directly
  |
  = note: to create a recursive async fn, you must rewrite to return a boxed future
  = for more information, see https://rust-lang.github.io/async-book/index.html
