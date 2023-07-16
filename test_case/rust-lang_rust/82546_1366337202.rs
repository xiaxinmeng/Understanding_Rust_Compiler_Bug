
error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
 --> /home/ubuntu/test.rs:1:45
  |
1 | fn test(i: &i32) -> std::thread::JoinHandle<_> {
  |                     ------------------------^-
  |                     |                       |
  |                     |                       not allowed in type signatures
  |                     help: replace with the correct return type: `JoinHandle<()>`
