
$ ./x.py test --stage 1 src/test/ui --test-args extern-use-primitive-type
...
running 1 test
F
failures:

---- [ui] ui/use/issue-60976-extern-use-primitive-type.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/projects/rust-1.36/build/x86_64-apple-darwin/stage1/bin/rustc" "/projects/rust-1.36/src/test/ui/use/issue-60976-extern-use-primitive-type.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--o
ut-dir" "/projects/rust-1.36/build/x86_64-apple-darwin/test/ui/use/issue-60976-extern-use-primitive-type" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/projects/rust-1.36/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/projects/rust-1.36/build/x86_64-apple-darwin/test/ui/use/issue-60976-extern-use-primitive-type/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src/librustc/hir/def.rs:340: attempted .def_id() on invalid res: PrimTy(Uint(u32))

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath


------------------------------------------



failures:
    [ui] ui/use/issue-60976-extern-use-primitive-type.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5575 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


command did not execute successfully: "/projects/rust-1.36/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/projects/rust-1.36/build/x86_64-apple-darwin/stage1/lib" "--run-lib-path" "/projects/rust-1.36/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/projects/rust-1.36/build/x86_64-apple-darwin/stage1/bin/rustc" "--src-base" "/projects/rust-1.36/src/test/ui" "--build-base" "/projects/rust-1.36/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage1-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/projects/rust-1.36/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/projects/rust-1.36/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/projects/rust-1.36/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/local/bin/gdb" "--lldb-version" "lldb-1100.0.30.6\nApple Swift version 5.1.2 (swiftlang-1100.0.278 clang-1100.0.33.9)\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "extern-use-primitive-type" "--quiet" "--llvm-version" "8.0.0-rust-1.36.0-dev-2c5656ae5\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /projects/rust-1.36/build/bootstrap/debug/bootstrap test --stage 1 src/test/ui --test-args extern-use-primitive-type
Build completed unsuccessfully in 0:00:23
