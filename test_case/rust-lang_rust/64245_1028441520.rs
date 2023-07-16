
~/Temp/foo> cargo test --target x86_64-unknown-linux-musl -Zdoctest-xcompile
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/x86_64-unknown-linux-musl/debug/deps/foo-65396ef18f7af6ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests foo

running 1 test
test src/lib.rs - abc (line 1) ... FAILED

failures:

---- src/lib.rs - abc (line 1) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/lib.rs:3:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
