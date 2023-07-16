
$ x test src/tools/rustfmt
Mismatch at src/lib.rs:25:
 use std::cell::RefCell;
 use std::collections::HashMap;
 use std::fmt;
-use std::io:: {self, Write};
+use std::io::{self, Write};
 use std::mem;
 use std::panic;
 use std::path::PathBuf;
test test::self_tests ... FAILED
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:357:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::self_tests

test result: FAILED. 154 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.96s
