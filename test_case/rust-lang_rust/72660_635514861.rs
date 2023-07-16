
warning: constant item is never used: `TEST_2`
 --> src/main.rs:2:1
  |
2 | const TEST_2: bool = TEST;
  |       ^^^^^^

error: using `TEST` will cause "index out of bound" error
 --> src/main.rs:1:20
  |
1 | const TEST: bool = [true][1];
  |       ----         ^^^^^^^^^ index out of bounds: the len is 1 but the index is 1
  |       |
  |       using this will cause "index out of bounds" error
  |
  = note: `#[deny(const_err)]` on by default

error: aborting due to previous error
