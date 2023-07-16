plain
travis_time:end:16854810:start=1545374257501318227,finish=1545374258508511900,duration=1007193673
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:48] .................................................................................................... 500/5192
[00:59:52] ..............................i..................................................................... 600/5192
[00:59:55] .................................................................................................... 700/5192
[01:00:01] .................................................................................................... 800/5192
[01:00:06] ......i...............i...FFFF.FFFF................................................................. 900/5192
[01:00:09] ..............................iiiii................................................................. 1000/5192
[01:00:14] .................................................................................................... 1200/5192
[01:00:17] .................................................................................................... 1300/5192
[01:00:19] .................................................................................................... 1400/5192
[01:00:22] .................................................................................................... 1500/5192
---
[01:02:33] failures:
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] 
[01:02:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:33] 
[01:02:33] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:02:33] 
[01:02:33] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] 
[01:02:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:33] 
[01:02:33] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:02:33] 
[01:02:33] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] 
[01:02:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:33] 
[01:02:33] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:02:33] 
[01:02:33] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] 
[01:02:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:33] 
[01:02:33] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:02:33] 
[01:02:33] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpgraph-trait-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] 
[01:02:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:33] 
[01:02:33] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:02:33] 
[01:02:33] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] 
[01:02:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:33] 
[01:02:33] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:02:33] 
[01:02:33] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:02:33] 
[01:02:33] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:02:33] 
[01:02:33] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:02:33] status: exit code: 101
[01:02:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:02:33] ------------------------------------------
[01:02:33] 
[01:02:33] ------------------------------------------
[01:02:33] stderr:
[01:02:33] stderr:
[01:02:33] ------------------------------------------
[01:02:33] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[01:02:33] 
[01:02:33] error: internal compiler error: unexpected panic
[01:02:33] 
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02:33] note: the compiler unexpectedly panicked. this is a bug.
[01:02unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:33] 
[01:02:33] 
[01:02:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:33] Build completed unsuccessfully in 0:03:59
[01:02:33] Build completed unsuccessfully in 0:03:59
[01:02:33] Makefile:58: recipe for target 'check' failed
[01:02:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16c7d3de
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 07:40:21 UTC 2018
---
travis_time:end:0296bd3e:start=1545378021960917318,finish=1545378021968093038,duration=7175720
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b7c8932
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025e5d00
travis_time:start:025e5d00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000f280a
$ dmesg | grep -i kill
