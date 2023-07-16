
---- [run-make] run-make-fulldeps\rustdoc-io-error stdout ----
error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[1]: Entering directory '/c/projects/rust/src/test/run-make-fulldeps/rustdoc-io-error'
make[1]: Leaving directory '/c/projects/rust/src/test/run-make-fulldeps/rustdoc-io-error'
------------------------------------------
stderr:
------------------------------------------
make[1]: *** No targets.  Stop.
------------------------------------------
thread '[run-make] run-make-fulldeps\rustdoc-io-error' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:3284:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-make] run-make-fulldeps\rustdoc-io-error
test result: FAILED. 192 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:503:22
