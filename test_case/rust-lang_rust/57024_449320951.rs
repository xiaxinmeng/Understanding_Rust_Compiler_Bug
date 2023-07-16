plain
travis_time:end:1d92d598:start=1545379651603177952,finish=1545379652678256611,duration=1075078659
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:54:12] .................................................................................................... 500/5192
[00:54:15] ..............................i..................................................................... 600/5192
[00:54:18] .................................................................................................... 700/5192
[00:54:23] .................................................................................................... 800/5192
[00:54:27] ......i...............i...FFFFF.FFF................................................................. 900/5192
[00:54:30] ..............................iiiii................................................................. 1000/5192
[00:54:36] .................................................................................................... 1200/5192
[00:54:38] .................................................................................................... 1300/5192
[00:54:41] .................................................................................................... 1400/5192
[00:54:43] .................................................................................................... 1500/5192
---
[00:56:33] .................................................................................................... 4800/5192
[00:56:36] .................................................................................................... 4900/5192
[00:56:39] .................................................................................................... 5000/5192
[00:56:41] .................................................................................................... 5100/5192
nown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] stderr:
[00:56:44] stderr:
[00:56:44] ------------------------------------------
[00:56:44] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[00:56:44] 
[00:56:44] error: internal compiler error: unexpected panic
[00:56:44] 
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] 
[00:56:44] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:44] 
[00:56:44] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:56:44] 
[00:56:44] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
---
[00:56:te a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:44] 
[00:56:44] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:56:44] 
[00:56:44] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] 
[00:56:44] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[00:56:44] 
[00:56:44] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:44] status: exit code: 101
[00:56:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] stderr:
[00:56:44] stderr:
[00:56:44] ---------------------------------------ers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] stderr:
[00:56:44] stderr:
[00:56:44] ------------------------------------------
[00:56:44] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[00:56:44] 
[00:56:44] error: internal compiler error: unexpected panic
[00:56:44] 
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] 
[00:56:44] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:44] 
[00:56:44] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:56:44] 
[00:56:44] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] 
[00:56:44] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[00:56:44] 
[00:56:44] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:44] status: exit code: 101
[00:56:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] stderr:
[00:56:44] stderr:
[00:56:44] ------------------------------------------
[00:56:44] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[00:56:44] 
[00:56:44] error: internal compiler error: unexpected panic
[00:56:44] 
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] 
[00:56:44] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:44] 
[00:56:44] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:56:44] 
[00:56:44] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] 
[00:56:44] ---- [ui] ui/dep-gruery-dep-graph -C prefer-dynamic -C rpath
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:44] 
[00:56:44] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[00:56:44] 
[00:56:44] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:44] status: exit code: 101
[00:56:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] stderr:
[00:56:44] stderr:
[00:56:44] ------------------------------------------
[00:56:44] thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:162:9
[00:56:44] 
[00:56:44] error: internal compiler error: unexpected panic
[00:56:44] 
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] note: the compiler unexpectedly panicked. this is a bug.
[00:56:44] 
[00:56:44] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:44] 
[00:56:44] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:56:44] 
[00:56:44] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:56:44] 
[00:56:44] ------------------------------------------
[00:56:44] 
[00:56:44] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
---
[00:56:44] 
[00:56:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:56:44] 
[00:56:44] 
[00:56:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:44] 
[00:56:44] 
[00:56:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:44] Build completed unsuccessfully in 0:03:39
[00:56:44] Build completed unsuccessfully in 0:03:39
[00:56:44] make: *** [check] Error 1
[00:56:44] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27a228f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 094-unknown-linux-gnu/release
---
24092 ./src/tools/lldb/packages/Python/lldbsuite
23856 ./src/tools/lldb/packages/Python/lldbsuite/test
23704 ./src/llvm-emscripten/test/tools
23524 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
23520 ./ocrashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:118c3dab
travis_time:start:118c3dab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:113e09a8
$ dmesg | grep -i kill
