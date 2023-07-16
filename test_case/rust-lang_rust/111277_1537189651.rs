plain
..............................................i...............................

failures:

---- src/io/mod.rs - io::IoSlice<'a>::into_bytes (line 1358) stdout ----
error[E0599]: no method named `as_bytes` found for struct `IoSlice<'_>` in the current scope
  |
  |
9 | let tail = io_slice.as_bytes()[3..];
  |                     ^^^^^^^^ method not found in `IoSlice<'_>`

error[E0599]: no method named `as_bytes` found for struct `IoSlice<'_>` in the current scope
   |
   |
14 | assert_eq!(io_slice.as_bytes(), b"def");
   |                     ^^^^^^^^ method not found in `IoSlice<'_>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/io/mod.rs - io::IoSlice<'a>::into_bytes (line 1358)

test result: FAILED. 1116 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 14.65s

error: doctest failed, to rerun pass `-p std --doc`
