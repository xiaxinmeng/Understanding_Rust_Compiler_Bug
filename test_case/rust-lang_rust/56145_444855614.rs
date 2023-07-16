plain
travis_time:end:0ad13af8:start=1544096341158726535,finish=1544096344133678538,duration=2974952003
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
472 ./src/libstd/sys/unix
468 ./src/test/ui/resolve
460 ./src/test/ui/rust-2018
448 ./src/test/ui/traits
448 ./src/test/ui/re_rebalance_coherence
444 ./src/test/ui/privacy
440 ./src/librustc_resolve
424 ./src/doc/unstable-book
416 ./src/test/ui/associated-types
---
[00:48:59] .................................................................................................... 1100/5166
[00:49:01] .................................................................................................... 1200/5166
[00:49:03] .................................................................................................... 1300/5166
[00:49:06] .................................................................................................... 1400/5166
[00:49:08] ..............................................................................F..................... 1500/5166
[00:49:15] .................................................................................................... 1700/5166
[00:49:18] .................................................................................................... 1800/5166
[00:49:21] .................................................................................................... 1900/5166
[00:49:24] ...................................i................................................................ 2000/5166
---
[00:50:24] ...........................................i........................................................ 3700/5166
[00:50:25] ..................................................................................................i. 3800/5166
[00:50:26] .................................................................................................... 3900/5166
[00:50:32] .................................................................................................... 4000/5166
[00:50:37] ........................................................................F........................... 4100/5166
[00:50:43] .................................................................................................... 4300/5166
[00:50:47] ...............................................i.................................................... 4400/5166
[00:50:53] .................................................................................................... 4500/5166
[00:50:56] .................................................................................................... 4600/5166
---
[00:51:12] .................................................................................................... 5100/5166
[00:51:14] .....i............................................................
[00:51:14] failures:
[00:51:14] 
[00:51:14] ---- [ui] ui/feature-gates/feature-gate-re-rebalance-coherence.rs stdout ----
[00:51:14] 
[00:51:14] error: /checkout/src/test/ui/feature-gates/feature-gate-re-rebalance-coherence.rs:10: unexpected error: '10:1: 10:70: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`) [E0210]'
[00:51:14] error: 1 unexpected errors found, 0 expected errors not found
[00:51:14] status: exit code: 1
[00:51:14] status: exit code: 1
[00:51:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-re-rebalance-coherence.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-re-rebalance-coherence/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-re-rebalance-coherence/auxiliary" "-A" "unused"
[00:51:14]     Error {
[00:51:14]         line_num: 10,
[00:51:14]         kind: Some(
[00:51:14]             Error
[00:51:14]             Error
[00:51:14]         ),
[00:51:14]         msg: "10:1: 10:70: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`) [E0210]"
[00:51:14] ]
[00:51:14] 
[00:51:14] 
[00:51:14] thread '[ui] ui/feature-gates/feature-gate-re-rebalance-coherence.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:51:14] 
[00:51:14] 
[00:51:14] ---- [ui] ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs stdout ----
[00:51:14] 
[00:51:14] error: /checkout/src/test/ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs:23: unexpected error: '23:1: 23:46: only traits defined in the current crate can be implemented for arbitrary types [E0117]'
[00:51:14] error: 1 unexpected errors found, 0 expected errors not found
[00:51:14] status: exit code: 1
[00:51:14] status: exit code: 1
[00:51:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1/auxiliary" "-A" "unused"
[00:51:14]     Error {
[00:51:14]         line_num: 23,
[00:51:14]         kind: Some(
[00:51:14]             Error
[00:51:14]             Error
[00:51:14]         ),
[00:51:14]         msg: "23:1: 23:46: only traits defined in the current crate can be implemented for arbitrary types [E0117]"
[00:51:14] ]
[00:51:14] 
[00:51:14] 
[00:51:14] thread '[ui] ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:51:14] 
[00:51:14] failures:
[00:51:14] failures:
[00:51:14]     [ui] ui/feature-gates/feature-gate-re-rebalance-coherence.rs
[00:51:14]     [ui] ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs
[00:51:14] test result: FAILED. 5140 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
[00:51:14] 
[00:51:14] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:51:14] 
[00:51:14] 
[00:51:14] 
[00:51:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:14] 
[00:51:14] 
[00:51:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:14] Build completed unsuccessfully in 0:04:03
[00:51:14] Build completed unsuccessfully in 0:04:03
[00:51:14] make: *** [check] Error 1
[00:51:14] Makefile:58: recipe for target 'check' failed
2411660 ./obj
2411620 ./obj/build
1765148 ./obj/build/x86_64-unknown-linux-gnu
1166296 ./src
---
143648 ./obj/build/x86_64-unknown-linux-gnu/test/ui
134920 ./.git/modules
134916 ./.git/modules/src
134748 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134744 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f7c44fwnbf-18uojqq-jerzeai9r9z3
130776 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130772 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125088 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125084 ./obj/build/x8
