plain
travis_time:end:0628e9c8:start=1546801187670098636,finish=1546801257708837494,duration=70038738858
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:17] .................................................................................................... 500/5298
[00:58:20] ...............................i.................................................................... 600/5298
[00:58:24] .................................................................................................... 700/5298
[00:58:29] .................................................................................................... 800/5298
[00:58:34] .......................................................................i...............i.....FFFFFFF 900/5298
[00:58:38] .F...............................................................................................iii 1000/5298
[00:58:44] .................................................................................................... 1200/5298
[00:58:46] .................................................................................................... 1300/5298
[00:58:49] .................................................................................................... 1400/5298
[00:58:52] .................................................................................................... 1500/5298
---
[01:01:07] failures:
[01:01:07] 
[01:01:07] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:01:07] 
[01:01:07] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:01:07] status: exit code: 101
[01:01:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] stderr:
[01:01:07] stderr:
[01:01:07] ------------------------------------------
[01:01:07] thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:999:5
[01:01:07] 
[01:01:07] error: internal compiler error: unexpected panic
[01:01:07] 
[01:01:07] note: the compiler unexpectedly panicked. this is a bug.
[01:01:07] note: the compiler unexpectedly panicked. this is a bug.
[01:01:07] 
[01:01:07] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:01:07] 
[01:01:07] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:01:07] 
[01:01:07] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:07] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:07] 
[01:01:07] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:01:07] 
[01:01:07] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:01:07] status: exit code: 101
[01:01:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] stderr:
[01:01:07] stderr:
[01:01:07] ------------------------------------------
[01:01:07] thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:999:5
[01:01:07] 
[01:01:07] error: internal compiler error: unexpected panic
[01:01:07] 
[01:01:07] note: the compiler unexpectedly panicked. this is a bug.
[01:01:07] note: the compiler unexpectedly panicked. this is a bug.
[01:01:07] 
[01:01:07] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:01:07] 
[01:01:07] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:01:07] 
[01:01:07] note: compiler flags: -Z ui-testingompile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:07] 
[01:01:07] 
[01:01:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:07] Build completed unsuccessfully in 0:04:04
[01:01:07] Build completed unsuccessfully in 0:04:04
[01:01:07] make: *** [check] Error 1
[01:01:07] Makefile:48: recipe for target 'check' failed
2559672 ./obj
2559632 ./obj/build
1899812 ./obj/build/x86_64-unknown-linux-gnu
1135312 ./src
---
170844 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib
160340 ./obj/build/bootstrap/debug/incremental
153288 ./src/tools/clang
144240 ./obj/build/bootstrap/debug/incremental/bootstrap-19b7zdw1vl8vg
144236 ./obj/build/bootstrap/debug/incremental/bootstrap-19b7zdw1vl8vg/s-f8amqr5h24-wkqhly-3qyhkhoj3k9mu
138104 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
138100 ./obj/build/x86_64-unkn
