console
$ cargo +nightly -Vv
cargo 1.50.0-nightly (75d5d8cff 2020-12-22)
release: 1.50.0
commit-hash: 75d5d8cffe3464631f82dcd3c470b78dc1dda8bb
commit-date: 2020-12-22
$ cargo +nightly check
    Checking rust-issue v0.1.0 (/tmp/rust-issue)
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
2 |     Some(0i32)
  |     ^^^^^^^^^^ expected `()`, found enum `Option`
  |
  = note: expected unit type `()`
                  found enum `Option<i32>`
help: try adding a semicolon
  |
2 |     Some(0i32);
  |               ^
help: try adding a return type
  |
1 | fn expected_unit_got_option_i32() -> Option<i32> {
  |                                   ^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rust-issue`

To learn more, run the command again with --verbose.
$ cargo +stage1 check
    Checking rust-issue v0.1.0 (/tmp/rust-issue)
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn expected_unit_got_option_i32() {
  |                                   - possibly return type missing here?
2 |     Some(0i32)
  |     ^^^^^^^^^^- help: try adding a semicolon: `;`
  |     |
  |     expected `()`, found enum `Option`
  |
  = note: expected unit type `()`
                  found enum `Option<i32>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rust-issue`

To learn more, run the command again with --verbose.
