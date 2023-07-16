plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:03] 
[00:55:03] running 51 tests
[00:55:14] ERROR 2018-07-18T19:51:29Z: compiletest::runtest: None
[00:55:23] .................................F.................
[00:55:23] 
[00:55:23] ---- [mir-opt] mir-opt/nll/named-lifetimes-basic.rs stdout ----
[00:55:23] ---- [mir-opt] mir-opt/nll/named-lifetimes-basic.rs stdout ----
[00:55:23] thread '[mir-opt] mir-opt/nll/named-lifetimes-basic.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:23] Current block: None
[00:55:23] Actual Line: "| \'_#0r    | {bb0[0..=1], \'_#0r}"
[00:55:23] Expected Line: "| \'_#0r    | {\'_#0r, bb0[0..=1]}"
[00:55:23] Test Name: rustc.use_x.nll.0.mir
[00:55:23] Expected:
[00:55:23] ... (elided)
[00:55:23] | Free Region Mapping
[00:55:23] | '_#0r    | Global   | ['_#2r, '_#1r, '_#0r, '_#4r, '_#3r]
[00:55:23] | '_#1r    | External | ['_#1r, '_#4r]
[00:55:23] | '_#2r    | External | ['_#2r, '_#1r, '_#4r]
[00:55:23] | '_#3r    | Local    | ['_#4r, '_#3r]
[00:55:23] | '_#4r    | Local    | ['_#4r]
[00:55:23] | Inferred Region Values
[00:55:23] | Inferred Region Values
[00:55:23] | '_#0r    | {'_#0r, bb0[0..=1]}
[00:55:23] | '_#1r    | {'_#1r, bb0[0..=1]}
[00:55:23] | '_#2r    | {'_#2r, bb0[0..=1]}
[00:55:23] | '_#3r    | {'_#3r, bb0[0..=1]}
[00:55:23] | '_#4r    | {'_#4r, bb0[0..=1]}
[00:55:23] | '_#5r    | {'_#1r, bb0[0..=1]}
[00:55:23] | '_#6r    | {'_#2r, bb0[0..=1]}
[00:55:23] | '_#7r    | {'_#1r, bb0[0..=1]}
[00:55:23] | '_#8r    | {'_#3r, bb0[0..=1]}
[00:55:23] |
[00:55:23] ... (elided)
[00:55:23] fn use_x(_1: &'_#5r mut i32, _2: &'_#6r u32, _3: &'_#7r u32, _4: &'_#8r u32) -> bool {
[00:55:23] Actual:
[00:55:23] | Free Region Mapping
[00:55:23] | '_#0r    | Global   | ['_#2r, '_#1r, '_#0r, '_#4r, '_#3r]
[00:55:23] | '_#1r    | External | ['_#1r, '_#4r]
[00:55:23] | '_#2r    | External | ['_#2r, '_#1r, '_#4r]
[00:55:23] | '_#3r    | Local    | ['_#4r, '_#3r]
[00:55:23] | '_#4r    | Local    | ['_#4r]
[00:55:23] | Inferred Region Values
[00:55:23] | Inferred Region Values
[00:55:23] | '_#0r    | {bb0[0..=1], '_#0r}
[00:55:23] | '_#1r    | {bb0[0..=1], '_#1r}
[00:55:23] | '_#2r    | {bb0[0..=1], '_#2r}
[00:55:23] | '_#3r    | {bb0[0..=1], '_#3r}
[00:55:23] | '_#4r    | {bb0[0..=1], '_#4r}
[00:55:23] | '_#5r    | {bb0[0..=1], '_#1r}
[00:55:23] | '_#6r    | {bb0[0..=1], '_#2r}
[00:55:23] | '_#7r    | {bb0[0..=1], '_#1r}
[00:55:23] | '_#8r    | {bb0[0..=1], '_#3r}
[00:55:23] | Inference Constraints
[00:55:23] | Inference Constraints
[00:55:23] | '_#0r live at {bb0[0..=1], '_#0r}
[00:55:23] | '_#1r live at {bb0[0..=1], '_#1r}
[00:55:23] | '_#2r live at {bb0[0..=1], '_#2r}
[00:55:23] | '_#3r live at {bb0[0..=1], '_#3r}
[00:55:23] | '_#4r live at {bb0[0..=1], '_#4r}
[00:55:23] | '_#1r: '_#5r due to All
[00:55:23] | '_#1r: '_#7r due to All
[00:55:23] | '_#2r: '_#6r due to All
[00:55:23] | '_#3r: '_#8r due to All
[00:55:23] | '_#5r: '_#1r due to All
[00:55:23] | '_#6r: '_#2r due to All
[00:55:23] | '_#7r: '_#1r due to All
[00:55:23] | '_#8r: '_#3r due to All
[00:55:23] fn use_x(_1: &'_#5r mut i32, _2: &'_#6r u32, _3: &'_#7r u32, _4: &'_#8r u32) -> bool{
[00:55:23]     let mut _0: bool;
[00:55:23]     bb0: {                              
[00:55:23]                                          | Live variables on entry to bb0[0]: []
[00:55:23]         _0 = const true;
[00:55:23]                                          | Live variables on entry to bb0[1]: []
[00:55:23]         return;
[00:55:23]     | Live variables on exit from bb0: []
[00:55:23] }', tools/compiletest/src/runtest.rs:2815:13
[00:55:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:23] 
[00:55:23] 
---
[00:55:23] 
[00:55:23] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:23] 
[00:55:23] 
[00:55:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:23] 
[00:55:23] 
[00:55:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:23] Build completed unsuccessfully in 0:10:13
[00:55:23] Build completed unsuccessfully in 0:10:13
[00:55:23] Makefile:58: recipe for target 'check' failed
[00:55:23] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2f07b67c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00c96990:start=1531943500378562483,finish=1531943500385241510,duration=6679027
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15a56886
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b511bec
travis_time:start:2b511bec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:055783a7
$ dmesg | grep -i kill
