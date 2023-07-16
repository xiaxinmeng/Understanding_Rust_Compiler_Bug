
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage1 tool compiletest (x86_64-unknown-linux-gnu)
Check compiletest compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 1 test
test [compile-fail] compile-fail/associated-type-projection-from-multiple-supertraits.rs ... FAILED

failures:

---- [compile-fail] compile-fail/associated-type-projection-from-multiple-supertraits.rs stdout ----

error: /home/nash/code/rust/src/test/compile-fail/associated-type-projection-from-multiple-supertraits.rs:35: unexpected "note": 'missing associated type `Color` value'

error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: /home/nash/code/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc /home/nash/code/rust/src/test/compile-fail/associated-type-projection-from-multiple-supertraits.rs -L /home/nash/code/rust/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/nash/code/rust/build/x86_64-unknown-linux-gnu/test/compile-fail/associated-type-projection-from-multiple-supertraits.stage1-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/nash/code/rust/build/x86_64-unknown-linux-gnu/test/compile-fail/associated-type-projection-from-multiple-supertraits.stage1-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/nash/code/rust/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 35,
        kind: Some(
            Note
        ),
        msg: "missing associated type `Color` value"
    }
]

thread '[compile-fail] compile-fail/associated-type-projection-from-multiple-supertraits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1082
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail/associated-type-projection-from-multiple-supertraits.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured



command did not execute successfully: "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/compiletest" "--compile-lib-path" "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--rustdoc-path" "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustdoc" "--src-base" "/home/nash/code/rust/src/test/compile-fail" "--build-base" "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/nash/code/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/home/nash/code/rust/build/x86_64-unknown-linux-gnu/rust-test-helpers" "--docck-python" "python" "--lldb-python" "python" "associated-type-projection-from-multiple-supertraits" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
expected success, got: exit code: 101
