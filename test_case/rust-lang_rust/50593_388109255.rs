plain
[00:51:17] ---- [run-pass] run-pass/nll/get_default.rs stdout ----
[00:51:17]  
[00:51:17] error: compilation failed!
[00:51:17] status: exit code: 101
[00:51:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/nll/get_default.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/nll/get_default.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/nll/get_default.stage2-x86_64-unknown-linux-gnu.aux"
[00:51:17] ------------------------------------------
[00:51:17] 
[00:51:17] ------------------------------------------
[00:51:17] stderr:
[00:51:17] stderr:
[00:51:17] ------------------------------------------
[00:51:17] error[E0499]: cannot borrow `*map` as mutable more than once at a time
[00:51:17]    |
[00:51:17]    |
[00:51:17] 16 |     match map.get_mut(&key) {
[00:51:17]    |           --- first mutable borrow occurs here
[00:51:17] ...
[00:51:17] 19 |             map.insert(key, "".to_string());
[00:51:17]    |             ^^^ second mutable borrow occurs here
[00:51:17]    |
[00:51:17] note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 15:1...
[00:51:17]    |
[00:51:17]    |
[00:51:17] 15 | / fn get_default(map: &mut HashMap<usize, String>, key: usize) -> &mut String {
[00:51:17] 16 | |     match map.get_mut(&key) {
[00:51:17] 17 | |         Some(value) => value,
[00:51:17] ...  |
[00:51:17] 22 | |     }
[00:51:17] 23 | | }
[00:51:17]    | |_^
[00:51:17]    | |_^
[00:51:17] 
[00:51:17] error[E0499]: cannot borrow `*map` as mutable more than once at a time
[00:51:17]    |
[00:51:17]    |
[00:51:17] 16 |     match map.get_mut(&key) {
[00:51:17]    |           --- first mutable borrow occurs here
[00:51:17] ...
[00:51:17] 20 |             map.get_mut(&key).unwrap()
[00:51:17]    |             ^^^ second mutable borrow occurs here
[00:51:17]    |
[00:51:17] note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 15:1...
[00:51:17]    |
[00:51:17]    |
[00:51:17] 15 | / fn get_default(map: &mut HashMap<usize, String>, key: usize) -> &mut String {
[00:51:17] 16 | |     match map.get_mut(&key) {
[00:51:17] 17 | |         Some(value) => value,
[00:51:17] ...  |
[00:51:17] 22 | |     }
[00:51:17] 23 | | }
[00:51:17]    | |_^
---
[00:51:17] 
[00:51:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:51:17] 
[00:51:17] 
[00:51:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:17] 
[00:51:17] 
[00:51:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:17] Build completed unsuccessfully in 0:12:30
[00:51:17] Build completed unsuccessfully in 0:12:30
[00:51:17] Makefile:58: recipe for target 'check' failed
[00:51:17] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:229e8fd0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
