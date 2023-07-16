
error[E0106]: missing lifetime specifier
 --> test.rs:1:32
  |
1 | fn foo(_: impl Iterator<Item = & u32>) {}
  |                                ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
 --> test.rs:1:7
  |
1 | fn foo(_: impl Iterator<Item = & u32>) {}
  |       ^
