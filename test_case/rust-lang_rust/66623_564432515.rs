
running 6 tests
test src/lib.rs - ByteOrder (line 114) ... FAILED
test src/lib.rs - ByteOrder (line 104) ... ok
test src/lib.rs -  (line 15) ... ok
test src/lib.rs -  (line 28) ... ok
test src/new.rs - new::ReadBytesExt (line 15) ... ok
test src/new.rs - new::WriteBytesExt (line 143) ... ok

failures:

---- src/lib.rs - ByteOrder (line 114) stdout ----
error: literal out of range for `i16`
 --> src/lib.rs:118:33
  |
7 | BigEndian::write_i16(&mut buf, -50_000);
  |                                 ^^^^^^
  |
  = note: `#[deny(overflowing_literals)]` on by default

error: literal out of range for `i16`
 --> src/lib.rs:119:13
  |
8 | assert_eq!(-50_000, BigEndian::read_i16(&buf));
  |             ^^^^^^

error: aborting due to 2 previous errors

Couldn't compile the test.

failures:
    src/lib.rs - ByteOrder (line 114)

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
