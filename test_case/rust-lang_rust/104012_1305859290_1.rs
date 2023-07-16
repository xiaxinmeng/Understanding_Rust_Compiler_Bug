console
error: unexpected closing delimiter: `}`
 --> bug3.rs:7:1
  |
1 | fn f(i: u32, j: u32) {
  |                      - this delimiter might not be properly closed...
...
6 |     }
  |     - ...as it matches this but it has different indentation
7 | }
  | ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
