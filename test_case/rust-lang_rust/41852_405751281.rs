
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/main.rs:2:26
  |
2 |     Some(()).into_iter().filter(|| false).collect();
  |                          ^^^^^^ -- takes 0 arguments
  |                          |
  |                          expected closure that takes 1 argument
