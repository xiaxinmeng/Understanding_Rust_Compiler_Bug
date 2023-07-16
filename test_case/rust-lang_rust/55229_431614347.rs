plain
[00:50:23] .................................................................................................... 2200/4660
[00:50:27] ....................................i............................................................... 2300/4660
[00:50:31] .................................................................................................... 2400/4660
[00:50:34] .................................................................................................... 2500/4660
[00:50:38] ...................................................iiiiiiiii........................................ 2600/4660
[00:50:44] .................................................................................................... 2800/4660
[00:50:48] .................................................................................................... 2900/4660
[00:50:51] .................................................................................i.................. 3000/4660
[00:50:54] .................................................................................................... 3100/4660
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:50] 
[01:03:50] running 111 tests
[01:03:53] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:03:53] ..iiii.....
[01:03:53] 
[01:03:53]  finished in 3.492
[01:03:53] travis_fold:end:test_codegen

---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:55] 
[01:03:55] running 92 tests
hes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
[01:04:08] ------------------------------------------
[01:04:08] 
[01:04:08] ------------------------------------------
[01:04:08] stderr:
[01:04:08] stderr:
[01:04:08] ------------------------------------------
[01:04:08] {"message":"`TypeckTables(add_type_ascription_to_parameter)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":2588,"byte_end":2699,"line_start":100,"line_end":103,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_type_ascription_to_parameter() {","highlight_start":1,"highlight_end":44},{"text":"    let closure = |x: u32| x + 1u32;","highlight_start":1,"highlight_end":37},{"text":"    let _: u32 = closure(1);","highlight_start":1,"highlight_end":29},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `TypeckTables(add_type_ascription_to_parameter)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:100:1\n   |\nLL | / pub fn add_type_ascription_to_parameter() {\nLL | |     let closure = |x: u32| x + 1u32;\nLL | |     let _: u32 = closure(1);\nLL | | }\n   | |_^\n\n"}
[01:04:08] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:08] ------------------------------------------
[01:04:08] 
[01:04:08] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:04:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:04:08] 
[01:04:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:04:08] 
[01:04:08] 
[01:04:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:08] 
[01:04:08] 
[01:04:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:08] Build completed unsuccessfully in 0:18:29
[01:04:08] Build completed unsuccessfully in 0:18:29
[01:04:08] Makefile:58: recipe for target 'check' failed
[01:04:08] make: *** [check] Error 1
75788 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
72532 ./src/llvm/lib
70532 ./obj/build/x86_64-unknown-linux-gnu/native
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
---
travis_time:end:0f82f73d:start=1540066258740294292,finish=1540066258747934118,duration=7639826
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25dd73bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashl
