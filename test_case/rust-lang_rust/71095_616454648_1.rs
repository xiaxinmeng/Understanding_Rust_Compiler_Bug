
failures:

---- [ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs stdout ----

The actual stderr differed from the expected stderr.
Actual stderr saved to /home/pickfire/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/alloc-types-no-impls-length-33.stderr
Actual stderr saved to /home/pickfire/rust/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.stderr

error: /home/pickfire/rust/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:15: expected error not found: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<[i32; 33]>` is not satisfied

error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 1
command: "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/home/pickfire/rust/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs""-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir""/home/pickfire/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/pickfire/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/auxiliary"
not found errors (from test file): [
    Error {
        line_num: 15,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<[i32; 33]>` is not satisfied",
    },
]

thread '[ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs' panickedat 'explicit panic', src/tools/compiletest/src/runtest.rs:1459:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    [ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs

test result: FAILED. 9831 passed; 1 failed; 59 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/home/pickfire/rust/src/test/ui" "--build-base" "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/pickfire/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--bless" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/pickfire/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/pickfire/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.1-rust-dev" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /home/pickfire/rust/build/bootstrap/debug/bootstrap test --bless
Build completed unsuccessfully in 0:24:57
