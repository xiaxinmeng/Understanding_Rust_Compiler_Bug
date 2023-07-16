
   Compiling playground v0.0.1 (/playground)
warning: constant item is never used: `TEST`
 --> src/main.rs:1:1
  |
1 | const TEST: bool = [true][1];
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: constant item is never used: `TEST_2`
 --> src/main.rs:2:1
  |
2 | const TEST_2: bool = TEST;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: any use of this value will cause an error
 --> src/main.rs:1:20
  |
1 | const TEST: bool = [true][1];
  | -------------------^^^^^^^^^-
  |                    |
  |                    index out of bounds: the len is 1 but the index is 1
  |
  = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
 --> src/main.rs:2:22
  |
2 | const TEST_2: bool = TEST;
  | ---------------------^^^^-
  |                      |
  |                      referenced constant has errors

error: aborting due to 2 previous errors

error: could not compile `playground`.

To learn more, run the command again with --verbose.
