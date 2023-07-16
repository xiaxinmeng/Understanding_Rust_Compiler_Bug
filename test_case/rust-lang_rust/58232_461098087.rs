plain
travis_time:end:0459b0f1:start=1549467911228625092,finish=1549467983416116908,duration=72187491816
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:27] .................................................................................................... 2500/5369
[01:04:30] .................................................................................................... 2600/5369
[01:04:34] .................................................................................................... 2700/5369
[01:04:38] .................................................................................................... 2800/5369
[01:04:42] ........................................................................................F........... 2900/5369
[01:04:49] .................................................................................................... 3100/5369
[01:04:53] .................................................................................................... 3200/5369
[01:04:56] .........................i.......................................................................... 3300/5369
[01:05:00] ...........................................................................................ii...i..i 3400/5369
---
[01:06:11] failures:
[01:06:11] 
[01:06:11] ---- [ui] ui/issues/issue-56128.rs stdout ----
[01:06:11] 
[01:06:11] error: test compilation failed although it shouldn't!
[01:06:11] status: exit code: 101
[01:06:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56128.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56128/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56128/auxiliary" "-A" "unused"
[01:06:11] ------------------------------------------
[01:06:11] 
[01:06:11] ------------------------------------------
[01:06:11] stderr:
[01:06:11] stderr:
[01:06:11] ------------------------------------------
[01:06:11] thread 'rustc' panicked at 'src/librustc/hir/map/collector.rs:261: inconsistent DepNode at `"/checkout/src/test/ui/issues/issue-56128.rs:7:9: 7:14"` for `PathSegment(PathSegment { ident: super#0, id: Some(NodeId(37)), hir_id: Some(HirId { owner: DefIndex(0:4), local_id: 2 }), def: Some(Err), args: None, infer_types: false })`: current_dep_node_owner=::bar[0]::{{?}}[1] (DefIndex(0:5)), hir_id.owner=::bar[0]::{{?}}[0] (DefIndex(0:4))', src/librustc/util/bug.rs:37:26
[01:06:11] 
[01:06:11] error: internal compiler error: unexpected panic
[01:06:11] 
[01:06:11] note: the compiler unexpectedly panicked. this is a bug.
[01:06:11] note: the compiler unexpectedly panicked. this is a bug.
[01:06:11] 
[01:06:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:11] 
[01:06:11] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:06:11] 
[01:06:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:06:11] 
[01:06:11] ------------------------------------------
[01:06:11] 
[01:06:11] thread '[ui] ui/issues/issue-56128.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:06:11] 
[01:06:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:06:11] 
[01:06:11] 
[01:06:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:11] 
[01:06:11] 
[01:06:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:11] Build completed unsuccessfully in 0:04:04
[01:06:11] Build completed unsuccessfully in 0:04:04
[01:06:11] Makefile:48: recipe for target 'check' failed
[01:06:11] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0634e7aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 16:52:52 UTC 2019
---
travis_time:end:07c74a1a:start=1549471973982662700,finish=1549471973987061922,duration=4399222
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:063d28c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ee8a153
travis_time:start:2ee8a153
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynami
