

---- [run-pass] run-pass/issue-54462-mutable-noalias-correctness.rs stdout ----


executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/run-pass/issue-54462-mutable-noalias-correctness.rs" "--target=s390x-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-pass/issue-54462-mutable-noalias-correctness/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-Ccodegen-units=1" "-O" "-L" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-pass/issue-54462-mutable-noalias-correctness/auxiliary"
------stdout------------------------------

------stderr------------------------------

------------------------------------------
executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-pass/issue-54462-mutable-noalias-correctness/a"
------stdout------------------------------

------stderr------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[9.0, 5.0, 1.0, 10.0, 6.0, 2.0, 11.0, 7.0, 3.0, 12.0, 8.0, 4.0]`,
 right: `[9.0, 1.0, 5.0, 10.0, 2.0, 6.0, 11.0, 3.0, 7.0, 12.0, 4.0, 4.0]`', /<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/run-pass/issue-54462-mutable-noalias-correctness.rs:32:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

error: test run failed!
status: exit code: 101
command: "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-pass/issue-54462-mutable-noalias-correctness/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[9.0, 5.0, 1.0, 10.0, 6.0, 2.0, 11.0, 7.0, 3.0, 12.0, 8.0, 4.0]`,
 right: `[9.0, 1.0, 5.0, 10.0, 2.0, 6.0, 11.0, 3.0, 7.0, 12.0, 4.0, 4.0]`', /<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/run-pass/issue-54462-mutable-noalias-correctness.rs:32:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/issue-54462-mutable-noalias-correctness.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/issue-54462-mutable-noalias-correctness.rs

test result: FAILED. 2914 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
