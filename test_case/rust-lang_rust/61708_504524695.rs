plain
travis_time:end:169207d4:start=1561137905728027532,finish=1561137906616550037,duration=888522505
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:56:23] ...........................................................................ii...i..ii............... 3600/5694
[00:56:27] .................................................................................................... 3700/5694
[00:56:30] .................................................................................................... 3800/5694
[00:56:34] .....................................................................................ii............. 3900/5694
[00:56:37] .............................................F...................................................... 4000/5694
[00:56:41] ........................................................................i........................... 4200/5694
[00:56:43] .................................................................................................... 4300/5694
[00:56:51] .................................................................................................... 4400/5694
[00:57:01] .................................................................................................... 4500/5694
---
[00:57:45] .................................................................................................... 5600/5694
[00:57:48] ................................i.............................................................
[00:57:48] failures:
[00:57:48] 
[00:57:48] ---- [ui] ui/or-patterns/mix-with-wild.rs stdout ----
[00:57:48] 
[00:57:48] error: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10: unexpected error: '10:14: 10:19: src/librustc_mir/build/matches/test.rs:916: simplifyable pattern found: Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:19, kind: Or { pats: [Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:15, kind: Constant { value: Const { ty: usize, val: Scalar(0x0000000000000000) } } }, Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:18: 10:19, kind: Wild }] } }'
[00:57:48] error: 1 unexpected errors found, 0 expected errors not found
[00:57:48] status: exit code: 101
[00:57:48] status: exit code: 101
[00:57:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/mix-with-wild.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mix-with-wild" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/mix-with-wild/auxiliary" "-A" "unused"
[00:57:48]     Error {
[00:57:48]         line_num: 10,
[00:57:48]         kind: Some(
[00:57:48]             Error,
[00:57:48]             Error,
[00:57:48]         ),
[00:57:48]         msg: "10:14: 10:19: src/librustc_mir/build/matches/test.rs:916: simplifyable pattern found: Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:19, kind: Or { pats: [Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:14: 10:15, kind: Constant { value: Const { ty: usize, val: Scalar(0x0000000000000000) } } }, Pattern { ty: usize, span: /checkout/src/test/ui/or-patterns/mix-with-wild.rs:10:18: 10:19, kind: Wild }] } }",
[00:57:48] ]
[00:57:48] 
[00:57:48] 
[00:57:48] thread '[" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:48] 
[00:57:48] 
[00:57:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:48] Build completed unsuccessfully in 0:53:09
---
travis_time:end:03c6f9f0:start=1561141387160878554,finish=1561141387165681576,duration=4803022
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02532850
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; t
