plain
travis_fold:end:step_upload_script
travis_fold:start:worker_info
Worker information
hostname: 600c5fbe-4d35-42a4-b1b0-8b0da6048aff@1.production-2-worker-org-b-1-gce
version: v4.1.0-17-gbb60707 https://github.com/travis-ci/worker/tree/bb607075d490bc4e77c266194569bdd17ae6b7a6
startup: 6.438663479s
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:37] 
[01:00:37] running 90 tests
[01:00:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:00:50] ...............................F..........................................................
[01:00:50] 
[01:00:50] ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
[01:00:50] 
[01:00:50] 
[01:00:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:00:50] status: exit code: 1
[01:00:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
[01:00:50] ------------------------------------------
[01:00:50] 
[01:00:50] ------------------------------------------
[01:00:50] stderr:
[01:00:50] stderr:
[01:00:50] ------------------------------------------
[01:00:50] {"message":"`MirOptimized(add_type)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/let_expressions.rs","byte_start":1549,"byte_end":1594,"line_start":54,"line_end":56,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_type() {","highlight_start":1,"highlight_end":20},{"text":"    let _x: u32 = 2u32;","highlight_start":1,"highlight_end":24},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirOptimized(add_type)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/let_expressions.rs:54:1\n   |\nLL | / pub fn add_type() {\nLL | |     let _x: u32 = 2u32;\nLL | | }\n   | |_^\n\n"}
[01:00:50] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:50] ------------------------------------------
[01:00:50] 
[01:00:50] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:00:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:00:50] test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:00:50] 
[01:00:50] 
[01:00:50] 
[01:00:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:50] 
[01:00:50] 
[01:00:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:50] Build completed unsuccessfully in 0:15:58
[01:00:50] Build completed unsuccessfully in 0:15:58
[01:00:50] Makefile:58: recipe for target 'check' failed
[01:00:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3bc5ff58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0b9d37c0:start=1536612375682292453,finish=1536612375688791404,duration=6498951
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29b1268c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b301410
travis_time:start:0b301410
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a2bc804
$ dmesg | grep -i kill
