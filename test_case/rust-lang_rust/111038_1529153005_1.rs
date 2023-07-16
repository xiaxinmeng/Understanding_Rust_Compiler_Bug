rust
error[E0381]: used binding `x` isn't initialized
 --> /home/gh-compiler-errors/test.rs:4:9
  |
3 |         let x: &str;
  |             - binding declared here but left uninitialized
4 |         x
  |         ^ `x` used here but it isn't initialized
  |
help: consider assigning a value
  |
3 |         let x: &str = todo!();
  |                     +++++++++

note: erroneous constant used
 --> /home/gh-compiler-errors/test.rs:2:14
  |
2 |     println!("{}", {
  |              ^^^^
