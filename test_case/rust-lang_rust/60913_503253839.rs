plain
travis_time:end:1d028060:start=1560881125182430054,finish=1560881126060616750,duration=878186696
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:05:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:122: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:188: trailing whitespace
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:195: trailing whitespace
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1443: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2187: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2251: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:2287: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:75: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:603: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:68: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:74: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:306: trailing whitespace
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/promote_consts.rs:321: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/check_unsafety.rs:414: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:208: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:443: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:799: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:439: line longer than 100 chars
[00:05:08] some tidy checks failed
[00:05:08] 
[00:05:08] 
[00:05:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:08] 
[00:05:08] 
[00:05:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:08] Build completed unsuccessfully in 0:01:14
---
travis_time:end:0da639fe:start=1560881446069898737,finish=1560881446075630834,duration=5732097
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0174d6ca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:091ab063
travis_time:start:091ab063
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17307460
$ dmesg | grep -i kill
