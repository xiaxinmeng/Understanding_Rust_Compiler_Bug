plain
travis_time:end:16259ba6:start=1559658097696605921,finish=1559658186654318424,duration=88957712503
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:55] tidy error: /checkout/src/librustc_mir/transform/qualify_min_const_fn.rs:302: line longer than 100 chars
[00:04:55] tidy error: /checkout/src/test/ui/consts/single_variant_match_ice.rs:18: line longer than 100 chars
[00:04:59] some tidy checks failed
[00:04:59] 
[00:04:59] 
[00:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:59] 
[00:04:59] 
[00:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:59] Build completed unsuccessfully in 0:01:10
---
travis_time:end:14beffc8:start=1559658496971012938,finish=1559658496975710709,duration=4697771
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:007e587c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07e2be84
travis_time:start:07e2be84
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ba51308
$ dmesg | grep -i kill
