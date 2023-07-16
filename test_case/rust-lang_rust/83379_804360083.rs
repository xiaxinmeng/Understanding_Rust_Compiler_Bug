
error: unused return value of `init` that must be used
 --> test.rs:4:5
  |
4 |     init();
  |     ^^^^^^^
  |
note: the lint level is defined here
 --> test.rs:1:9
  |
1 | #![deny(unused_must_use)]
  |         ^^^^^^^^^^^^^^^
note: init() returns a drop guard
 --> test.rs:4:5
  |
4 |     init();
  |     ^^^^^^^

error: aborting due to previous error
