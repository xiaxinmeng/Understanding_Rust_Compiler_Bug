
[01:25:56] failures:
[01:25:56] 
[01:25:56] ---- [run-pass] run-pass/backtrace-debuginfo.rs stdout ----
[01:25:56] 	
[01:25:56] error: test run failed!
[01:25:56] status: exit code: 101
[01:25:56] command: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass/backtrace-debuginfo.stage2-i686-apple-darwin"
[01:25:56] stdout:
[01:25:56] ------------------------------------------
[01:25:56] 
[01:25:56] ------------------------------------------
[01:25:56] stderr:
[01:25:56] ------------------------------------------
[01:25:56] thread 'main' panicked at 'trace does not match position list: test case 0
[01:25:56] thread 'main' panicked at 'explicit panic', /Users/travis/build/rust-lang/rust/src/test/run-pass/backtrace-debuginfo.rs:78:4
[01:25:56] stack backtrace:
[01:25:56]    0:   0x1e88c8 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h58fac47b4a42cd77
[01:25:56]    1:   0x1e28f8 - std::sys_common::backtrace::_print::h285e13909861b307
[01:25:56]    2:   0x1f58ae - std::panicking::default_hook::{{closure}}::h41f407050fbb9ee1
[01:25:56]    3:   0x1f5490 - std::panicking::default_hook::h1733bcbadb978adb
[01:25:56]    4:   0x1f5ef1 - std::panicking::rust_panic_with_hook::hfa1cfe4ac54c174c
[01:25:56]    5:    0xa7fdd - std::panicking::begin_panic::h02a6a28c53c09dd1
[01:25:56]    6:    0xa9822 - backtrace_debuginfo::inner::hd9078f7dd9efd109
[01:25:56]    7:    0xa9942 - backtrace_debuginfo::outer::hb361f2c852aa11c2
[01:25:56]    8:    0xaa0d2 - backtrace_debuginfo::main::hda900a635795a7e5
[01:25:56]    9:   0x207a71 - __rust_maybe_catch_panic
[01:25:56]   10:   0x1f6bd9 - std::rt::lang_start::ha0d04e85043cf9de
[01:25:56]   11:    0xaafc0 - main
[01:25:56] 
[01:25:56] ---
[01:25:56] backtrace-debuginfo.rs:119
[01:25:56] backtrace-debuginfo.rs:167
[01:25:56] ', /Users/travis/build/rust-lang/rust/src/test/run-pass/backtrace-debuginfo.rs:133:4
[01:25:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:56] 
[01:25:56] ------------------------------------------
[01:25:56] 
[01:25:56] thread '[run-pass] run-pass/backtrace-debuginfo.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2435:8
[01:25:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:56] 
[01:25:56] 
[01:25:56] failures:
[01:25:56]     [run-pass] run-pass/backtrace-debuginfo.rs
[01:25:56] 
[01:25:56] test result: FAILED. 2745 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
[01:25:56] 
[01:25:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:323:21
[01:25:56] 
[01:25:56] 
[01:25:56] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools/i686-apple-darwin/release/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-pass" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass" "--stage-id" "stage2-i686-apple-darwin" "--mode" "run-pass" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.9.1/bin/node" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-360.1.70" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "4.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:25:56] expected success, got: exit code: 101
[01:25:56] 
[01:25:56] 
[01:25:56] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:25:56] Build completed unsuccessfully in 0:21:42
[01:25:56] make: *** [check] Error 1
