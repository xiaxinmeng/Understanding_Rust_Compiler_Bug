plain
travis_time:end:001a62b0:start=1552236711816129107,finish=1552236713994114483,duration=2177985376
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:13:35] .................................................................................................... 5100/5460
[01:13:39] .................................................................................................... 5200/5460
[01:13:42] .................................................................................................... 5300/5460
[01:13:45] ...............................................................................................i.... 5400/5460
[01:13:47] ........................................F...................
[01:13:47] 
[01:13:47] ---- [ui] ui/wf/wf-unsafe-trait-obj-match.rs stdout ----
[01:13:47] 
[01:13:47] error: /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:23: unexpected error: '23:17: 23:19: match arms have incompatible types [E0308]'
[01:13:47] error: /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:23: unexpected error: '23:17: 23:19: match arms have incompatible types [E0308]'
[01:13:47] 
[01:13:47] error: /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:26: unexpected error: '26:21: 26:23: the trait `Trait` cannot be made into an object [E0038]'
[01:13:47] error: /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:21: expected error not found: E0308
[01:13:47] 
[01:13:47] error: /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:25: expected error not found: E0308
[01:13:47] 
[01:13:47] 
[01:13:47] error: 2 unexpected errors found, 2 expected errors not found
[01:13:47] status: exit code: 1
[01:13:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-unsafe-trait-obj-match/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-unsafe-trait-obj-match/auxiliary" "-A" "unused"
[01:13:47]     Error {
[01:13:47]         line_num: 23,
[01:13:47]         kind: Some(
[01:13:47]             Error
---
[01:13:47]         line_num: 26,
[01:13:47]         kind: Some(
[01:13:47]             Error
[01:13:47]         ),
[01:13:47]         msg: "26:21: 26:23: the trait `Trait` cannot be made into an object [E0038]"
[01:13:47] ]
[01:13:47] 
[01:13:47] not found errors (from test file): [
[01:13:47]     Error {
---
[01:13:47] 
[01:13:47] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:47] 
[01:13:47] 
[01:13:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:47] 
[01:13:47] 
[01:13:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:47] Build completed unsuccessfully in 0:04:22
[01:13:47] Build completed unsuccessfully in 0:04:22
[01:13:47] Makefile:48: recipe for target 'check' failed
[01:13:47] make: *** [check] Error 1
