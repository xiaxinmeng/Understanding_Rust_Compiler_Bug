
failures:

---- num/mod.rs - num::i128::rotate_left (line 48) stdout ----
thread 'num/mod.rs - num::i128::rotate_left (line 48)' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `375243667947399698977780`,
 right: `1333138420`', num/mod.rs:7:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', src/librustdoc/test.rs:358:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- num/mod.rs - num::i128::rotate_right (line 49) stdout ----
thread 'num/mod.rs - num::i128::rotate_right (line 49)' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `24592063248569515181595825140`,
 right: `26522252353395899526533775249656270710`', num/mod.rs:7:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', src/librustdoc/test.rs:358:17

---- num/mod.rs - num::u128::rotate_left (line 48) stdout ----
thread 'num/mod.rs - num::u128::rotate_left (line 48)' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `26522252377987868550572336102567333750`,
 right: `1333138420`', num/mod.rs:7:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', src/librustdoc/test.rs:358:17

---- num/mod.rs - num::u128::rotate_right (line 49) stdout ----
thread 'num/mod.rs - num::u128::rotate_right (line 49)' panicked at 'test executable failed:

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `87368559493120`,
 right: `26522252353395899526533775249656270710`', num/mod.rs:7:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.

', src/librustdoc/test.rs:358:17


failures:
    num/mod.rs - num::i128::rotate_left (line 48)
    num/mod.rs - num::i128::rotate_right (line 49)
    num/mod.rs - num::u128::rotate_left (line 48)
    num/mod.rs - num::u128::rotate_right (line 49)

test result: FAILED. 2195 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--doc'
