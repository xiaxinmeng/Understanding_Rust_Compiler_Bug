plain
travis_time:end:04e32ecb:start=1558614264863172290,finish=1558614267013671483,duration=2150499193
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:127: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:193: trailing whitespace
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:200: trailing whitespace
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:274: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:310: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1469: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2171: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2288: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2324: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:72: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:71: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:77: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:306: trailing whitespace
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:321: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:314: trailing whitespace
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:429: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:188: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:411: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:768: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:443: line longer than 100 chars
[00:04:49] tidy error: /checkout/src/librustc_mir/dataflow/move_paths/mod.rs:258: trailing whitespace
[00:04:49] tidy error: /checkout/src/librustc/mir/tcx.rs:124: line longer than 100 chars
[00:04:54] some tidy checks failed
[00:04:54] 
[00:04:54] 
[00:04:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:54] 
[00:04:54] 
[00:04:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:54] Build completed unsuccessfully in 0:01:14
[00:04:54] Build completed unsuccessfully in 0:01:14
[00:04:54] make: *** [tidy] Error 1
[00:04:54] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2bee6a75
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 23 12:29:32 UTC 2019
---
travis_time:end:1330fae8:start=1558614573618322429,finish=1558614573624105285,duration=5782856
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00a56bc0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ba5a54c
travis_time:start:1ba5a54c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:083fe03e
$ dmesg | grep -i kill
