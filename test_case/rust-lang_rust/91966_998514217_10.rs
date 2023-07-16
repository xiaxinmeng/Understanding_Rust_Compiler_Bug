
error[E0308]: mismatched types
 --> src/main.rs:5:5
  |
5 |     func(f);
  |     ^^^^ lifetime mismatch
  |
  = note: expected reference `&i32`
             found reference `&i32`
note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
 --> src/main.rs:4:13
  |
4 |     let f = |i: &i32| i;
  |             ^^^^^^^^^^^
note: the lifetime requirement is introduced here
 --> src/main.rs:1:29
  |
1 | fn func(_: impl Fn(&i32) -> &i32) {}
  |                             ^^^^
