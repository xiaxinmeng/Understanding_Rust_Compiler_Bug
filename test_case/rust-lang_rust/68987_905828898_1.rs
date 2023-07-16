
error: unexpected closing delimiter: `}`
 --> src/lib.rs:8:1
  |
2 | fn f(i: u32, j: u32) {
  |                      - this opening brace...
...
7 |     }
  |     - ...matches this closing brace
8 | }
  | ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
 --> src/lib.rs:6:28
  |
5 |     while cnt < j {
  |                   - unclosed delimiter
6 |         write!&mut res, " ");
  |                            ^ mismatched closing delimiter

error: aborting due to 2 previous errors
