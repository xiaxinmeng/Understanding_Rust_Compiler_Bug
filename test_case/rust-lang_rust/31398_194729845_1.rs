
% DYLD_LIBRARY_PATH=$(pwd)/x86_64-apple-darwin/stage1/lib:$(pwd)/x86_64-apple-darwin/llvm/Release/lib:$DYLD_LIBRARY_PATH x86_64-apple-darwin/stage1/bin/compiletest --compile-lib-path x86_64-apple-darwin/stage1/lib --run-lib-path x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib --rustc-path x86_64-apple-darwin/stage1/bin/rustc  --aux-base $(pwd)/../src/test/auxiliary/ --stage-id stage1-x86_64-apple-darwin --target x86_64-apple-darwin --host x86_64-apple-darwin --python "/Users/fklock/bin/python2.7"  --host-rustcflags "   --cfg rtopt   -C rpath -L x86_64-apple-darwin/rt" --lldb-python-dir=/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python --target-rustcflags "   --cfg rtopt   -C rpath -L x86_64-apple-darwin/rt" command-before-exec  --src-base /Users/fklock/Dev/Mozilla/rust-mir/src/test/run-pass/ --build-base x86_64-apple-darwin/test/run-pass/ --mode run-pass --rustdoc-path x86_64-apple-darwin/stage1/bin/rustdoc --android-cross-path=

running 1 test
test [run-pass] run-pass/command-before-exec.rs ... FAILED

failures:

---- [run-pass] run-pass/command-before-exec.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/command-before-exec.stage1-x86_64-apple-darwin 
stdout:
------------------------------------------
stderr: Ok("dyld: Symbol not found: __ZN11collections4hash5table16round_up_to_next10_FILE_LINE20h6b6ee708a299a5faiacE\n  Referenced from: /Users/fklock/Dev/Mozilla/rust-mir/objdir-dbgopt/x86_64-apple-darwin/test/run-pass/command-before-exec.stage1-x86_64-apple-darwin\n  Expected in: /Users/fklock/Dev/Mozilla/rust-mir/objdir-dbgopt/x86_64-apple-darwin/stage1/lib/libstd-9026086f.dylib\n in /Users/fklock/Dev/Mozilla/rust-mir/objdir-dbgopt/x86_64-apple-darwin/test/run-pass/command-before-exec.stage1-x86_64-apple-darwin\n")

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: output.stderr.is_empty()', /Users/fklock/Dev/Mozilla/rust-mir/src/test/run-pass/command-before-exec.rs:62
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/command-before-exec.rs' panicked at 'explicit panic', /Users/fklock/Dev/Mozilla/rust-mir/src/compiletest/runtest.rs:1651
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/command-before-exec.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /Users/fklock/Dev/Mozilla/rust-mir/src/compiletest/compiletest.rs:246
