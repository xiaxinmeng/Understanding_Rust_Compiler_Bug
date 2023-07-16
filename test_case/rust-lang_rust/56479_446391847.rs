plain
travis_time:end:313d2edb:start=1544565133629305681,finish=1544565327666977381,duration=194037671700
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:26] .....................................................................................i.............. 3100/5170
[00:49:29] .................................................................................................... 3200/5170
[00:49:32] ................................................ii..i..ii........................................... 3300/5170
[00:49:36] .................................................................................................... 3400/5170
[00:49:39] .F...............................................................................F.................. 3500/5170
[00:49:44] ...................................................i................................................ 3700/5170
[00:49:45] .................................................................................................... 3800/5170
[00:49:47] .......i............................................................................................ 3900/5170
[00:49:51] .................................................................................................... 4000/5170
---
[00:50:36] failures:
[00:50:36] 
[00:50:36] ---- [ui] ui/nll/issue-55394.rs stdout ----
[00:50:36] 
[00:50:36] error: /checkout/src/test/ui/nll/issue-55394.rs:21: unexpected error: '21:9: 21:20: lifetime may not live long enough'
[00:50:36] error: /checkout/src/test/ui/nll/issue-55394.rs:21: expected error not found: unsatisfied lifetime constraints
[00:50:36] 
[00:50:36] error: 1 unexpected errors found, 1 expected errors not found
[00:50:36] status: exit code: 1
[00:50:36] status: exit code: 1
[00:50:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55394.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/auxiliary" "-A" "unused"
[00:50:36]     Error {
[00:50:36]         line_num: 21,
[00:50:36]         kind: Some(
[00:50:36]             Error
[00:50:36]             Error
[00:50:36]         ),
[00:50:36]         msg: "21:9: 21:20: lifetime may not live long enough"
[00:50:36] ]
[00:50:36] 
[00:50:36] not found errors (from test file): [
[00:50:36]     Error {
---
[00:50:36] thread '[ui] ui/nll/issue-55394.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1349:13
[00:50:36] 
[00:50:36] ---- [ui] ui/nll/user-annotations/issue-54124.rs stdout ----
[00:50:36] 
[00:50:36] error: /checkout/src/test/ui/nll/user-annotations/issue-54124.rs:4: unexpected error: '4:22: 4:23: lifetime may not live long enough'
[00:50:36] 
[00:50:36] error: /checkout/src/test/ui/nll/user-annotations/issue-54124.rs:4: unexpected error: '4:22: 4:23: lifetime may not live long enough'
[00:50:36] error: /checkout/src/test/ui/nll/user-annotations/issue-54124.rs:4: expected error not found: unsatisfied lifetime constraints
[00:50:36] 
[00:50:36] error: /checkout/src/test/ui/nll/user-annotations/issue-54124.rs:4: expected error not found: unsatisfied lifetime constraints
[00:50:36] 
[00:50:36] 
[00:50:36] error: 2 unexpected errors found, 2 expected errors not found
[00:50:36] status: exit code: 1
[00:50:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/issue-54124.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-54124/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-54124/auxiliary" "-A" "unused"
[00:50:36]     Error {
[00:50:36]         line_num: 4,
[00:50:36]         kind: Some(
[00:50:36]             Error
[00:50:36]             Error
[00:50:36]         ),
[00:50:36]         msg: "4:22: 4:23: lifetime may not live long enough"
[00:50:36]     Error {
[00:50:36]         line_num: 4,
[00:50:36]         kind: Some(
[00:50:36]             Error
[00:50:36]             Error
[00:50:36]         ),
[00:50:36]         msg: "4:22: 4:23: lifetime may not live long enough"
[00:50:36] ]
[00:50:36] 
[00:50:36] not found errors (from test file): [
[00:50:36]     Error {
---
[00:50:36]         line_num: 4,
[00:50:36]         kind: Some(
[00:50:36]             Error
[00:50:36]         ),
[00:ython" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:36] 
[00:50:36] 
[00:50:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:36] Build completed unsuccessfully in 0:03:58
[00:50:36] Build completed unsuccessfully in 0:03:58
[00:50:36] Makefile:58: recipe for target 'check' failed
[00:50:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:079cf6bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 11 22:46:12 UTC 2018
