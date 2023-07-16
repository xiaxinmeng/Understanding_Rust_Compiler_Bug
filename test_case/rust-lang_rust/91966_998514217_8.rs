
error[E0308]: mismatched types
 --> src/main.rs:6:5
  |
6 |     func(f);
  |     ^^^^ lifetime mismatch
  |
  = note: expected reference `&i32`
             found reference `&i32`
note: the lifetime requirement is introduced here
 --> src/main.rs:1:29
  |
1 | fn func(_: impl Fn(&i32) -> &i32) {}
  |                             ^^^^
