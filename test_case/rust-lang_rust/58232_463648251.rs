plain
travis_time:end:160d5bcc:start=1550150691763516033,finish=1550150692715615405,duration=952099372
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:06:55] .................................................................................................... 5000/5387
[01:06:59] .................................................................................................... 5100/5387
[01:07:02] .................................................................................................... 5200/5387
[01:07:04] .................................................................................................... 5300/5387
x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56128/auxiliary" "-A" "unused"
[01:07:07] ------------------------------------------
[01:07:07] 
[01:07:07] ------------------------------------------
[01:07:07] stderr:
[01:07:07] stderr:
[01:07:07] ------------------------------------------
[01:07:07] thread 'rustc' panicked at 'src/librustc/hir/map/collector.rs:261: inconsistent DepNode at `"/checkout/src/test/ui/issues/issue-56128.rs:7:9: 7:14"` for `PathSegment(PathSegment { ident: super#0, id: Some(NodeId(37)), hir_id: Some(HirId { owner: DefIndex(0:4), local_id: 2 }), def: Some(Err), args: None, infer_types: false })`: current_dep_node_owner=::bar[0]::{{?}}[1] (DefIndex(0:5)), hir_id.owner=::bar[0]::{{?}}[0] (DefIndex(0:4))', src/librustc/util/bug.rs:37:26
[01:07:07] 
[01:07:07] error: internal compiler error: unexpected panic
[01:07:07] 
[01:07:07] note: the compiler unexpectedly panicked. this is a bug.
[01:07:07] note: the compiler unexpectedly panicked. this is a bug.
[01:07:07] 
[01:07:07] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:07] 
[01:07:07] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:07] 
[01:07:07] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:07:07] 
[01:07:07] ------------------------------------------
[01:07:07] 
[01:07:07] thread '[ui] ui/issues/issue-56128.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:07:07] 
[01:07:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:07:07] 
[01:07:07] 
[01:07:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:07] 
[01:07:07] 
[01:07:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:07] Build completed unsuccessfully in 0:04:09
[01:07:07] Build completed unsuccessfully in 0:04:09
[01:07:07] make: *** [check] Error 1
[01:07:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05032f3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 14:32:14 UTC 2019
