plain
travis_time:end:0a9a559a:start=1558093555781497353,finish=1558093645388304309,duration=89606806956
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:127: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:193: trailing whitespace
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:200: trailing whitespace
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:274: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:310: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1468: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2170: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2287: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2323: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:72: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:306: trailing whitespace
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:321: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:317: trailing whitespace
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:434: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:188: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:414: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:771: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:446: line longer than 100 chars
[00:03:57] tidy error: /checkout/src/librustc_mir/dataflow/move_paths/mod.rs:257: trailing whitespace
[00:03:57] tidy error: /checkout/src/librustc/mir/tcx.rs:124: line longer than 100 chars
[00:04:02] some tidy checks failed
[00:04:02] 
[00:04:02] 
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:02] 
[00:04:02] 
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:02] Build completed unsuccessfully in 0:01:12
[00:04:02] Build completed unsuccessfully in 0:01:12
[00:04:02] Makefile:67: recipe for target 'tidy' failed
[00:04:02] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:030c9513
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 11:51:36 UTC 2019
---
travis_time:end:144a8464:start=1558093897710639669,finish=1558093897715228884,duration=4589215
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15fb11aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05727e33
travis_time:start:05727e33
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:224c1513
$ dmesg | grep -i kill
