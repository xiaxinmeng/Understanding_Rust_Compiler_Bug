plain
[00:44:42] ....................................................................................................
[00:44:49] ....................................................................................................
[00:44:55] ....................................................................................................
[00:45:03] ....................................i...............................................................
[00:45:09] ............i.........................................F.............................................
[00:45:23] ....................................................................................................
[00:45:30] ...........i....................................................................
[00:45:30] failures:
[00:45:30] 
[00:45:30] 
[00:45:30] ---- [ui] ui/raw_string_hash_count.rs stdout ----
[00:45:30]  
[00:45:30] error: /checkout/src/test/ui/raw_string_hash_count.rs:14: unexpected error: '14:5: 14:262: too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols'
[00:45:30] 
[00:45:30] error: /checkout/src/test/ui/raw_string_hash_count.rs:14: expected error not found: too many `#` characters
[00:45:30] 
[00:45:30] error: 1 unexpected errors found, 1 expected errors not found
[00:45:30] status: exit code: 101
[00:45:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw_string_hash_count.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:45:30] unexpected errors (from JSON output): [
[00:45:30]     Error {
[00:45:30]         line_num: 14,
[00:45:30]         kind: Some(
[00:45:30]             Error
[00:45:30]         ),
[00:45:30]         msg: "14:5: 14:262: too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols"
[00:45:30] ]
[00:45:30] 
[00:45:30] not found errors (from test file): [
[00:45:30]     Error {
[00:45:30]     Error {
[00:45:30]         line_num: 14,
[00:45:30]         kind: Some(
[00:45:30]             Error
[00:45:30]         ),
[00:45:30]         msg: "too many `#` characters"
[00:45:30] ]
[00:45:30] 
[00:45:30] thread '[ui] ui/raw_string_hash_count.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[00:45:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:30] 
[00:45:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:45:30] 
[00:45:30] 
[00:45:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:30] 
[00:45:30] 
[00:45:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:30] Build completed unsuccessfully in 0:02:45
[00:45:30] Build completed unsuccessfully in 0:02:45
[00:45:30] Makefile:58: recipe for target 'check' failed
[00:45:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:044c0ffa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
