
error[E0308]: mismatched types
 --> src/main.rs:7:13
  |
4 | fn foo() -> u8 {
  |             -- expected `u8` because of return type
...
7 |             break;
  |             ^^^^^
  |             |
  |             expected `u8`, found `()`
  |             help: give it a value of the expected type: `break 42`
