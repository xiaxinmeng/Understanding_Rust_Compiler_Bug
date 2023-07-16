shell
$ $ ./x.py test --stage 1
<output trimmed>
test [run-pass] run-pass-fulldeps/proc-macro/span-api-tests.rs ... FAILED

failures:

---- [run-pass] run-pass-fulldeps/proc-macro/span-api-tests.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/src/test/run-pass-fulldeps/proc-macro/span-api-tests.rs" "-L" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/span-api-tests.stage1-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/span-api-tests.stage1-x86_64-unknown-linux-gnu.run-pass.libaux"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'proc_macro::__internal::with_sess() called before set_parse_sess()!', src/libproc_macro/lib.rs:819:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: proc macro panicked
  --> /mnt/d/Users/coriolinus/Documents/Projects/rust-lang/src/test/run-pass-fulldeps/proc-macro/span-api-tests.rs:29:1
   |
29 | assert_source_file! { "Hello, world!" }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


------------------------------------------

thread '[run-pass] run-pass-fulldeps/proc-macro/span-api-tests.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2485:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass-fulldeps/proc-macro/span-api-tests.rs

test result: FAILED. 0 passed; 1 failed; 79 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:329:21


command did not execute successfully: "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/src/test/run-pass-fulldeps" "--build-base" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/mnt/d/Program Files/nodejs/node" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--llvm-version" "4.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /mnt/d/Users/coriolinus/Documents/Projects/rust-lang/build/bootstrap/debug/bootstrap test --stage 1
Build completed unsuccessfully in 0:01:17

$ git status; git rev-parse --short HEAD
On branch master
Your branch is up-to-date with 'origin/master'.

nothing to commit, working directory clean
f764eaf
