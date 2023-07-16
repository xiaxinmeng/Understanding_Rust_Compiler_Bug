
rustc 1.17.0-nightly (b1e31766d 2017-03-03)
error: type could implement `Copy`; consider adding `impl Copy`
 --> <anon>:4:5
  |
4 |       pub struct Foo {
  |  _____^ starting here...
5 | |         pub field: i32
6 | |     }
  | |_____^ ...ending here
  |
note: lint level defined here
 --> <anon>:1:9
  |
1 | #![deny(missing_copy_implementations)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
