
  rust git:(pull_52360) ./x.py test --keep-stage 0 --stage 1 src/test/pretty ; say "I am done. I am really done. ... Really"
Updating only changed submodules
Submodules updated in 0.09 seconds
    Finished dev [unoptimized] target(s) in 0.30s
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old codegen backend. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old libtest. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Warning: Using a potentially old codegen backend. This may not behave well.
Assembling stage1 compiler (x86_64-apple-darwin)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage1 rustc from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Warning: Using a potentially old libtest. This may not behave well.
Copying stage1 test from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 tool compiletest (x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.36s
Check compiletest suite=pretty mode=pretty (x86_64-apple-darwin -> x86_64-apple-darwin)

running 50 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiF
failures:

---- [pretty] pretty/issue_12590_c.rs stdout ----

error: pretty-printing failed in round 0 revision None
status: signal: 6
command: "/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage1/bin/rustc" "-" "-Z" "unpretty=expanded" "--target" "x86_64-apple-darwin" "-L" "/Users/tinco/Source/rust/build/x86_64-apple-darwin/test/pretty/issue_12590_c/auxiliary.pretty" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/tinco/Source/rust/build/x86_64-apple-darwin/native/rust-test-helpers"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
dyld: Library not loaded: @rpath/libtest-6187db25f3681564.dylib
  Referenced from: /Users/tinco/Source/rust/build/x86_64-apple-darwin/stage1/bin/rustc
  Reason: image not found

------------------------------------------

thread '[pretty] pretty/issue_12590_c.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [pretty] pretty/issue_12590_c.rs

test result: FAILED. 0 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22


command did not execute successfully: "/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage1/lib" "--run-lib-path" "/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage1/bin/rustc" "--src-base" "/Users/tinco/Source/rust/src/test/pretty" "--build-base" "/Users/tinco/Source/rust/build/x86_64-apple-darwin/test/pretty" "--stage-id" "stage1-x86_64-apple-darwin" "--mode" "pretty" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/tinco/Source/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/tinco/Source/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/local/bin/gdb" "--lldb-version" "lldb-902.0.79.7" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--quiet" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /Users/tinco/Source/rust/build/bootstrap/debug/bootstrap test --keep-stage 0 --stage 1 src/test/pretty
Build completed unsuccessfully in 0:00:03
