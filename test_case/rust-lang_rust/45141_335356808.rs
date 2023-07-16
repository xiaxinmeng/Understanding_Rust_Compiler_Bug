
[01:38:04] failures:
[01:38:04] 
[01:38:04] ---- [debuginfo-lldb] debuginfo/self-in-generic-default-method.rs stdout ----
[01:38:04] 	NOTE: compiletest thinks it is using LLDB version 360
[01:38:04] 
[01:38:04] error: Error while running LLDB
[01:38:04] status: signal: 11
[01:38:04] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/self-in-generic-default-method.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/self-in-generic-default-method.debugger.script"
[01:38:04] stdout:
[01:38:04] ------------------------------------------
[01:38:04] LLDB batch-mode script
[01:38:04] ----------------------
[01:38:04] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/self-in-generic-default-method.debugger.script'.
[01:38:04] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/self-in-generic-default-method.stage2-i686-apple-darwin'.
[01:38:04] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:38:04] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/self-in-generic-default-method.stage2-i686-apple-darwin'
[01:38:04] settings set auto-confirm true
[01:38:04] 
[01:38:04] version
[01:38:04] lldb-360.1.70 
[01:38:04] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:38:04] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:38:04] type category enable Rust
[01:38:04] 
[01:38:04] breakpoint set --file 'self-in-generic-default-method.rs' --line 131
[01:38:04] Breakpoint 1: 2 locations. 
[01:38:04] breakpoint set --file 'self-in-generic-default-method.rs' --line 136
[01:38:04] Breakpoint 2: 2 locations. 
[01:38:04] breakpoint set --file 'self-in-generic-default-method.rs' --line 141
[01:38:04] Breakpoint 3: where = self-in-generic-default-method.stage2-i686-apple-darwin`self_in_generic_default_method::Trait::self_owned<self_in_generic_default_method::Struct,f32> + 50 at self-in-generic-default-method.rs:141, address = 0x000020d2 
[01:38:04] quit
[01:38:04] 
[01:38:04] 
[01:38:04] ------------------------------------------
[01:38:04] stderr:
[01:38:04] ------------------------------------------
[01:38:04] 
[01:38:04] ------------------------------------------
[01:38:04] 
[01:38:04] thread '[debuginfo-lldb] debuginfo/self-in-generic-default-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2425:8
[01:38:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:38:04] 
[01:38:04] 
[01:38:04] failures:
[01:38:04]     [debuginfo-lldb] debuginfo/self-in-generic-default-method.rs
[01:38:04] 
[01:38:04] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[01:38:04] 
[01:38:04] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:323:21
[01:38:04] 
[01:38:04] 
[01:38:04] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools/i686-apple-darwin/release/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/debuginfo" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo" "--stage-id" "stage2-i686-apple-darwin" "--mode" "debuginfo-lldb" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.9.1/bin/node" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-360.1.70" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "4.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:38:04] expected success, got: exit code: 101
[01:38:04] 
[01:38:04] 
[01:38:04] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:38:04] Build completed unsuccessfully in 0:28:31
[01:38:04] make: *** [check] Error 1
[01:38:04] make: INTERNAL: Exiting with 1 jobserver tokens available; should be 2!
