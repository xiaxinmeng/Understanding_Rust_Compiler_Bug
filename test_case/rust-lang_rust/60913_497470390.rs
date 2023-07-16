plain
travis_time:end:002139d8:start=1559246404000686180,finish=1559246412344056122,duration=8343369942
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:30] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:127: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:193: trailing whitespace
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:200: trailing whitespace
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:268: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:304: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1471: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2288: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2324: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:75: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:607: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:71: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:77: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:306: trailing whitespace
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:321: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:314: trailing whitespace
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:429: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:188: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:411: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:768: line longer than 100 chars
[00:04:30] tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:439: line longer than 100 chars
[00:04:31] tidy error: /checkout/src/librustc/mir/tcx.rs:124: line longer than 100 chars
[00:04:35] some tidy checks failed
[00:04:35] 
[00:04:35] 
[00:04:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:35] 
[00:04:35] 
[00:04:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:35] Build completed unsuccessfully in 0:01:13
[00:04:35] Build completed unsuccessfully in 0:01:13
[00:04:35] make: *** [tidy] Error 1
[00:04:35] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3b204910
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 20:04:59 UTC 2019
---
travis_time:end:0acc0b48:start=1559246700478490688,finish=1559246700483605795,duration=5115107
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d8eeae0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:069d6c3b
travis_time:start:069d6c3b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13cb35fa
$ dmesg | grep -i kill
