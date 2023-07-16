plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:53] 
[01:01:53] running 89 tests
[01:02:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:02:14] .........................F...............................................................
[01:02:14] 
[01:02:14] ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
[01:02:14] 
[01:02:14] 
[01:02:14] error in revision `cfail1`: test compilation failed although it shouldn't!
[01:02:14] status: exit code: 101
[01:02:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
[01:02:14] ------------------------------------------
[01:02:14] 
[01:02:14] ------------------------------------------
[01:02:14] stderr:
[01:02:14] stderr:
[01:02:14] ------------------------------------------
[01:02:14] thread 'main' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', librustc/dep_graph/graph.rs:1074:9
[01:02:14] 
[01:02:14] error: internal compiler error: unexpected panic
[01:02:14] 
[01:02:14] 
[01:02:14] note: the compiler unexpectedly panicked. this is a bug.
[01:02:14] 
[01:02:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:14] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:02:14] 
[01:02:14] 
[01:02:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:02:14] 
[01:02:14] ------------------------------------------
[01:02:14] 
[01:02:14] thread '[incremental] incremental/hashes/function_interfaces.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[01:02:14] test result: FAILED. 88 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:02:14] 
[01:02:14] 
[01:02:14] 
[01:02:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:14] 
[01:02:14] 
[01:02:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:14] Build completed unsuccessfully in 0:16:16
[01:02:14] Build completed unsuccessfully in 0:16:16
[01:02:14] Makefile:58: recipe for target 'check' failed
[01:02:14] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03a56dfd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03ad1970:start=1529603689493976101,finish=1529603689500220931,duration=6244830
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06993b3d
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:039160ec
$ dmesg | grep -i kill
