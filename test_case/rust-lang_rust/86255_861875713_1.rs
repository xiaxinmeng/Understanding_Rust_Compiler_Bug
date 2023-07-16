
error[E0080]: any use of this value will cause an error
 --> src/main.rs:1:2
  |
1 | /  pub const X: () = {
2 | |     let y = [9; 12];
3 | | };
  | |__^ tried to allocate more memory than available to compiler
