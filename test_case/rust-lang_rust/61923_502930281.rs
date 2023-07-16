plain
travis_time:end:1c81285e:start=1560824301467688192,finish=1560824302229752957,duration=762064765
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:55] 
[01:03:55] running 9 tests
[01:03:55] iiiiiiiii
[01:03:55] 
[01:03:55]  finished in 0.146
[01:03:55] travis_fold:end:test_assembly


[01:03:55] travis_time:end:test_assembly:start=1560828147417072851,finish=1560828147563427265,duration=146354414

[01:03:55] travis_fold:start:test_incremental
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:55] 
[01:03:55] running 103 tests
[01:04:09] ................................................................................F................... 100/103
[01:04:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:10] F..
[01:04:10] failures:
[01:04:10] 
[01:04:10] 
[01:04:10] ---- [incremental] incremental/rlib_cross_crate/b.rs stdout ----
[01:04:10] 
[01:04:10] error in revision `rpass3`: compilation failed!
[01:04:10] status: exit code: 101
[01:04:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/rlib_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/auxiliary"
[01:04:10] ------------------------------------------
[01:04:10] 
[01:04:10] ------------------------------------------
[01:04:10] stderr:
[01:04:10] stderr:
[01:04:10] ------------------------------------------
[01:04:10] warning: unused variable: `x`
[01:04:10]   --> /checkout/src/test/incremental/rlib_cross_crate/b.rs:25:9
[01:04:10]    |
[01:04:10] LL |     let x: a::Y = 'c';
[01:04:10]    |         ^ help: consider prefixing with an underscore: `_x`
[01:04:10]    = note: #[warn(unused_variables)] on by default
[01:04:10] 
[01:04:10] 
[01:04:10] thread 'rustc' panicked at 'DepGraph::try_mark_previous_green() - Duplicate DepNodeColor insertion for check_mod_liveness(b[317d])', src/librustc/dep_graph/graph.rs:783:9
[01:04:10] 
[01:04:10] error: internal compiler error: unexpected panic
[01:04:10] 
[01:04:10] note: the compiler unexpectedly panicked. this is a bug.
[01:04:10] note: the compiler unexpectedly panicked. this is a bug.
[01:04:10] 
[01:04:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:10] 
[01:04:10] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:04:10] 
[01:04:10] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C rpath -C debuginfo=0
[01:04:10] 
[01:04:10] ------------------------------------------
[01:04:10] 
[01:04:10] 
[01:04:10] 
[01:04:10] ---- [incremental] incremental/type_alias_cross_crate/b.rs stdout ----
[01:04:10] 
[01:04:10] error in revision `rpass3`: compilation failed!
[01:04:10] status: exit code: 101
[01:04:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/type_alias_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/type_alias_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/type_alias_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/type_alias_cross_crate/b/auxiliary"
[01:04:10] ------------------------------------------
[01:04:10] 
[01:04:10] ------------------------------------------
[01:04:10] stderr:
[01:04:10] stderr:
[01:04:10] ------------------------------------------
[01:04:10] warning: unused variable: `x`
[01:04:10]   --> /checkout/src/test/incremental/type_alias_cross_crate/b.rs:19:9
[01:04:10]    |
[01:04:10] LL |     let x: a::Y = 'c';
[01:04:10]    |         ^ help: consider prefixing with an underscore: `_x`
[01:04:10]    = note: #[warn(unused_variables)] on by default
[01:04:10] 
[01:04:10] 
[01:04:10] thread 'rustc' panicked at 'DepGraph::try_mark_previous_green() - Duplicate DepNodeColor insertion for check_mod_liveness(b[317d])', src/librustc/dep_graph/graph.rs:783:9
[01:04:10] 
[01:04:10] error: internal compiler error: unexpected panic
[01:04:10] 
[01:04:10] note: the compiler unexpectedly panicked. this is a bug.
[01:04:10] note: the compiler unexpectedly panicked. this is a bug.
[01:04:10] 
[01:04:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:10] 
[01:04:10] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:04:10] 
[01:04:10] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
[01:04:10] 
[01:04:10] ------------------------------------------
[01:04:10] 
[01:04:10] 
---
[01:04:10] test result: FAILED. 101 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:10] 
[01:04:10] 
[01:04:10] 
[01:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:10] 
[01:04:10] 
[01:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:10] Build completed unsuccessfully in 0:59:23
---
travis_time:end:136f92c0:start=1560828165083095209,finish=1560828165087875601,duration=4780392
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:244134f2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ea01e60
travis_time:start:0ea01e60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:068c4839
$ dmesg | grep -i kill
