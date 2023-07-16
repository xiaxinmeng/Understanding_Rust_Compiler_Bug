plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:57] 
[00:50:57] running 90 tests
[00:51:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:09] ..F....F..............................................F.......F...........................
[00:51:09] 
[00:51:09] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[00:51:09] 
[00:51:09] 
[00:51:09] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:51:09] status: exit code: 101
[00:51:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[00:51:09] ------------------------------------------
[00:51:09] 
[00:51:09] ------------------------------------------
[00:51:09] stderr:
[00:51:09] stderr:
[00:51:09] ------------------------------------------
[00:51:09] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(point[8787])', librustc/ty/query/plumbing.rs:495:13
[00:51:09] 
[00:51:09] error: internal compiler error: unexpected panic
[00:51:09] 
[00:51:09] 
[00:51:09] note: the compiler unexpectedly panicked. this is a bug.
[00:51:09] 
[00:51:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:51:09] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:51:09] 
[00:51:09] 
[00:51:09] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -un with `RUST_BACKTRACE=1` for a backtrace.
[00:51:09] error: internal compiler error: unexpected panic
[00:51:09] 
[00:51:09] 
[00:51:09] note: the compiler unexpectedly panicked. this is a bug.
[00:51:09] 
[00:51:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:51:09] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:51:09] 
[00:51:09] 
[00:51:09] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:51:09] 
[00:51:09] ------------------------------------------
[00:51:09] 
[00:51:09] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:51:09] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:51:09] 
[00:51:09] ---- [incremental] incremental/issue-39828/issue-39828.rs stdout ----
[00:51:09] 
[00:51:09] error in revision `rpass2`: compilation failed!
[00:51:09] status: exit code: 101
[00:51:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39828/issue-39828.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/issue-39828.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/auxiliary"
[00:51:09] ------------------------------------------
[00:51:09] 
[00:51:09] ------------------------------------------
[00:51:09] stderr:
[00:51:09] stderr:
[00:51:09] ------------------------------------------
[00:51:09] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(generic[8787])', librustc/ty/query/plumbing.rs:495:13
[00:51:09] 
[00:51:09] error: internal compiler error: unexpected panic
[00:51:09] 
[00:51:09] 
[00:51:09] note: the compiler unexpectedly panicked. this is a bug.
[00:51:09] 
[00:51:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:51:09] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:51:09] 
[00:51:09] 
[00:51:09] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:51:09] 
[00:51:09] ------------------------------------------
[00:51:09] 
[00:51:09] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:51:09] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:51:09] 
[00:51:09] ---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----
[00:51:09] 
[00:51:09] error in revision `rpass2`: compilation failed!
[00:51:09] status: exit code: 101
[00:51:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/main.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
[00:51:09] ------------------------------------------
[00:51:09] 
[00:51:09] ------------------------------------------
[00:51:09] stderr:
[00:51:09] stderr:
[00:51:09] ------------------------------------------
[00:51:09] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(extern_crate[8787])', librustc/ty/query/plumbing.rs:495:13
[00:51:09] 
[00:51:09] error: internal compiler error: unexpected panic
[00:51:09] 
[00:51:09] 
[00:51:09] note: the compiler unexpectedly panicked. this is a bug.
[00:51:09] 
[00:51:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:51:09] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:51:09] 
[00:51:09] 
[00:51:09] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dyn" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:09] 
[00:51:09] 
[00:51:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:09] Build completed unsuccessfully in 0:09:41
[00:51:09] Build completed unsuccessfully in 0:09:41
[00:51:09] make: *** [check] Error 1
[00:51:09] Makefile:58: recipe for target 'check' failed
34716 ./obj/build/x86_64-unknown-linux-gnu/test/compile-fail
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34508 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
34056 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
