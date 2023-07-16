
[01:38:10] failures:
[01:38:10] 
[01:38:10] ---- [run-make] run-make/issue-26092 stdout ----
[01:38:10] 	
[01:38:10] error: make failed
[01:38:10] status: exit code: 2
[01:38:10] command: "make"
[01:38:10] stdout:
[01:38:10] ------------------------------------------
[01:38:10] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/issue-26092.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/issue-26092.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/issue-26092.stage2-x86_64-apple-darwin  -o "" blank.rs 2>&1 | \
[01:38:10] 			grep -i 'No such file or directory'
[01:38:10] 
[01:38:10] ------------------------------------------
[01:38:10] stderr:
[01:38:10] ------------------------------------------
[01:38:10] make[1]: *** [all] Error 1
[01:38:10] 
[01:38:10] ------------------------------------------
[01:38:10] 
[01:38:10] thread '[run-make] run-make/issue-26092' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2499:8
[01:38:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:38:10] 
[01:38:10] 
[01:38:10] failures:
[01:38:10]     [run-make] run-make/issue-26092
[01:38:10] 
[01:38:10] test result: [31mFAILED(B[m. 156 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:38:10] 
[01:38:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:323:21
