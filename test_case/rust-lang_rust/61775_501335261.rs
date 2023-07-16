plain
travis_time:end:1573ad64:start=1560354076917196095,finish=1560354079178142583,duration=2260946488
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:27] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:27] tidy error: /checkout/src/librustc_data_structures/graph/vec_graph/mod.rs: too many trailing newlines (2)
[00:04:27] tidy error: /checkout/src/librustc_mir/borrow_check/nll/pick_constraints.rs:1: TODO is deprecated; use FIXME
[00:04:27] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1488: TODO is deprecated; use FIXME
[00:04:27] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1495: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1500: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1507: XXX is deprecated; use FIXME
[00:04:27] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:2519: TODO is deprecated; use FIXME
[00:04:27] tidy error: /checkout/src/librustc/infer/canonical/mod.rs:310: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc/infer/region_constraints/mod.rs:976: line longer than 100 chars
[00:04:32] some tidy checks failed
[00:04:32] 
[00:04:32] 
[00:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:32] 
[00:04:32] 
[00:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:32] Build completed unsuccessfully in 0:01:12
---
travis_time:end:0003d492:start=1560354364239362990,finish=1560354364244276019,duration=4913029
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0216ec6a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e3d870f
travis_time:start:0e3d870f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:099dc04d
$ dmesg | grep -i kill
