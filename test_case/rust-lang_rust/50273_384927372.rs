plain
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:35] 
[01:03:35] running 12 tests
[01:03:45] .......F....
[01:03:45] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:03:45] 
[01:03:45] ---- [ui] ui-fulldeps/proc-macro/load-panic.rs stdout ----
[01:03:45]  diff of stderr:
[01:03:45] 
[01:03:45] 
[01:03:45] - thread '<unnamed>' panicked at 'nope!', $DIR/auxiliary/derive-panic.rs:22:5
[01:03:45] - note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:45] 3 error: proc-macro derive panicked
[01:03:45] 5    |
[01:03:45] 
[01:03:45] 
[01:03:45] The actual stderr differed from the expected stderr.
[01:03:45] The actual stderr differed from the expected stderr.
[01:03:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/load-panic.stderr
[01:03:45] To update references, run this command from build directory:
[01:03:45] /checkout/src/test/ui-fulldeps/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps' 'proc-macro/load-panic.rs'
[01:03:45] error: 1 errors occurred comparing output.
[01:03:45] status: exit code: 101
[01:03:45] status: exit code: 101
[01:03:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/load-panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/load-panic.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/load-panic.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:03:45] ------------------------------------------
[01:03:45] 
[01:03:45] ------------------------------------------
[01:03:45] stderr:
[01:03:45] stderr:
[01:03:45] ------------------------------------------
[01:03:45] error: proc-macro derive panicked
[01:03:45]   --> /checkout/src/test/ui-fulldeps/proc-macro/load-panic.rs:17:10
[01:03:45]    |
[01:03:45] LL | #[derive(A)]
[01:03:45]    |
[01:03:45]    |
[01:03:45]    = help: message: nope!
[01:03:45] error: aborting due to previous error
[01:03:45] 
[01:03:45] 
[01:03:45] ------------------------------------------
---
[01:03:45] test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:45] 
[01:03:45] 
[01:03:45] 
[01:03:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:45] 
[01:03:45] 
[01:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:45] Build completed unsuccessfully in 0:19:23
[01:03:45] Build completed unsuccessfully in 0:19:23
[01:03:45] make: *** [check] Error 1
[01:03:45] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:331d4a1d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
