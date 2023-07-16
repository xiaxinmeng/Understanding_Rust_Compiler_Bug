plain
............ii.................F.........................................

failures:

---- src/invalid_from_utf8.rs - invalid_from_utf8::INVALID_FROM_UTF8_UNCHECKED (line 17) stdout ----
error: calls to `std::str::from_utf8_unchecked` with a invalid literal -> is undefined behavior
 --> src/invalid_from_utf8.rs:20:5
  |
5 |     std::str::from_utf8_unchecked(b"Ru\x82st");
  |                                   |
  |                                   |
  |                                   the literal was valid UTF-8 up to the 2 bytes
  |
  = note: `#[deny(invalid_from_utf8_unchecked)]` on by default
error: aborting due to previous error

Couldn't compile the test.


failures:
    src/invalid_from_utf8.rs - invalid_from_utf8::INVALID_FROM_UTF8_UNCHECKED (line 17)
test result: FAILED. 70 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 3.23s


error: doctest failed, to rerun pass `-p rustc_lint --doc`
