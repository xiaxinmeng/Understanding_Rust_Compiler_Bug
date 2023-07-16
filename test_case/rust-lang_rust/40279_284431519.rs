bash
Check compiletest suite=run-fail mode=run-fail (i686-apple-darwin -> i686-apple-darwin)
running 126 tests
..F.........i.................................................................................................................
failures:
---- [run-fail] run-fail/assert-as-macro.rs stdout ----
	
error: compilation failed!
status: exit code: 101
command: /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc /Users/travis/build/rust-lang/rust/src/test/run-fail/assert-as-macro.rs -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail --target=i686-apple-darwin -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail/assert-as-macro.stage2-i686-apple-darwin.run-fail.libaux -C prefer-dynamic -o /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail/assert-as-macro.stage2-i686-apple-darwin -Crpath -O -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers
stdout:
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: linking with `cc` failed: exit code: 254
  |
  = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail/assert-as-macro.0.o" "-o" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail/assert-as-macro.stage2-i686-apple-darwin" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail/assert-as-macro.stage2-i686-apple-darwin.run-fail.libaux" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "-l" "std-de17a90db9a531ae" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-417d0cd8e787f49e.rlib" "-l" "System" "-l" "pthread" "-l" "c" "-l" "m" "-Wl,-rpath,@loader_path/../../stage2/lib/rustlib/i686-apple-darwin/lib" "-Wl,-rpath,/usr/local/lib/rustlib/i686-apple-darwin/lib"
  = note: ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail/assert-as-macro.stage2-i686-apple-darwin.run-fail.libaux'
          clang: error: unable to execute command: Segmentation fault: 11
          clang: error: linker command failed due to signal (use -v to see invocation)
          
error: aborting due to previous error
------------------------------------------
thread '[run-fail] run-fail/assert-as-macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2637
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-fail] run-fail/assert-as-macro.rs
test result: FAILED. 124 passed; 1 failed; 1 ignored; 0 measured
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:330
command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools/i686-apple-darwin/release/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-fail" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-fail" "--stage-id" "stage2-i686-apple-darwin" "--mode" "run-fail" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.9.1/bin/node" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-360.1.70" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "3.9.1\n" "--quiet" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
expected success, got: exit code: 101
Build completed unsuccessfully in 0:16:54
make: *** [check] Error 1
