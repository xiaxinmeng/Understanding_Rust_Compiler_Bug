plain
failures:

---- [run-pass-valgrind] src/test/run-pass-valgrind/cleanup-stdin.rs stdout ----

error: compilation failed!
status: signal: 9 (SIGKILL)
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/run-pass-valgrind/cleanup-stdin.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-O" "-C" "prefer-dynamic" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-pass-valgrind/cleanup-stdin/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-pass-valgrind/cleanup-stdin/auxiliary"
stdout: none
stderr: none


failures:
    [run-pass-valgrind] src/test/run-pass-valgrind/cleanup-stdin.rs
