rust
rustc 1.15.1 (021bd294c 2017-02-08)
error[E0308]: mismatched types
 --> <anon>:1:17
  |
1 |   fn foo() -> i32 {
  |  _________________^ starting here...
2 | |     22;
3 | | }
  | |_^ ...ending here: expected i32, found ()
  |
  = note: expected type `i32`
  = note:    found type `()`
help: consider removing this semicolon:
 --> <anon>:2:7
  |
2 |     22;
  |       ^

error[E0308]: mismatched types
 --> <anon>:5:18
  |
5 |   fn foo2() -> u32 {
  |  __________________^ starting here...
6 | |     22;
7 | | }
  | |_^ ...ending here: expected u32, found ()
  |
  = note: expected type `u32`
  = note:    found type `()`
help: consider removing this semicolon:
 --> <anon>:6:7
  |
6 |     22;
  |       ^

error: aborting due to 2 previous errors
