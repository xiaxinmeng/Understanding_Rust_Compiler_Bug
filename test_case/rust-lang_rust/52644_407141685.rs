plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:46] 
[00:50:46] running 90 tests
[00:50:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:58] ..F....F..............................................F.......F...........................
[00:50:58] 
[00:50:58] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[00:50:58] 
[00:50:58] 
[00:50:58] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:50:58] status: exit code: 101
[00:50:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] stderr:
[00:50:58] stderr:
[00:50:58] ------------------------------------------
[00:50:58] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(point[8787])', librustc/ty/query/plumbing.rs:495:13
[00:50:58] 
[00:50:58] error: internal compiler error: unexpected panic
[00:50:58] 
[00:50:58] 
[00:50:58] note: the compiler unexpectedly panicked. this is a bug.
[00:50:58] 
[00:50:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:58] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:58] 
[00:50:58] 
[00:50:58] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:58] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:58] 
[00:50:58] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[00:50:58] 
[00:50:58] error in revision `rpass2`: compilation failed!
[00:50:58] status: exit code: 101
[00:50:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/main.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] stderr:
[00:50:58] stderr:
[00:50:58] ------------------------------------------
[00:50:58] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(b[8787])', librustc/ty/query/plumbing.rs:495:13
[00:50:58] 
[00:50:58] error: internal compiler error: unexpected panic
[00:50:58] 
[00:50:58] 
[00:50:58] note: the compiler unexpectedly panicked. this is a bug.
[00:50:58] 
[00:50:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:58] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:58] 
[00:50:58] 
[00:50:58] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:58] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:58] 
[00:50:58] ---- [incremental] incremental/issue-39828/issue-39828.rs stdout ----
[00:50:58] 
[00:50:58] error in revision `rpass2`: compilation failed!
[00:50:58] status: exit code: 101
[00:50:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39828/issue-39828.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/issue-39828.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/auxiliary"
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] stderr:
[00:50:58] stderr:
[00:50:58] ------------------------------------------
[00:50:58] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(generic[8787])', librustc/ty/query/plumbing.rs:495:13
[00:50:58] 
[00:50:58] error: internal compiler error: unexpected panic
[00:50:58] 
[00:50:58] 
[00:50:58] note: the compiler unexpectedly panicked. this is a bug.
[00:50:58] 
[00:50:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:58] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:58] 
[00:50:58] 
[00:50:58] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:58] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:58] 
[00:50:58] ---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----
[00:50:58] 
[00:50:58] error in revision `rpass2`: compilation failed!
[00:50:58] status: exit code: 101
[00:50:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/main.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] stderr:
[00:50:58] stderr:
[00:50:58] ------------------------------------------
[00:50:58] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(extern_crate[8787])', librustc/ty/query/plumbing.rs:495:13
[00:50:58] 
[00:50:58] error: internal compiler error: unexpected panic
[00:50:58] 
[00:50:58] 
[00:50:58] note: the compiler unexpectedly panicked. this is a bug.
[00:50:58] 
[00:50:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:58] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:58] 
[00:50:58] 
[00:50:58] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] thread '[incremental] incremental/remapped_paths_cc/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
---
[00:50:58] test result: FAILED. 86 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[00:50:58] 
[00:50:58] 
[00:50:58] 
[00:50:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:58] 
[00:50:58] 
[00:50:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:58] Build completed unsuccessfully in 0:09:41
[00:50:58] Build completed unsuccessfully in 0:09:41
[00:50:58] make: *** [check] Error 1
[00:50:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03c32994
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0020f9f4:start=1532367750807167233,finish=1532367750814683040,duration=7515807
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08468d98
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08afbb96
travis_time:start:08afbb96
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0555d92b
$ dmesg | grep -i kill
