plain
travis_time:end:00b0fb4c:start=1554258020243611145,finish=1554258097549181194,duration=77305570049
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:30] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs
[00:03:30] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-extra-traits.rs
[00:03:30] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs
[00:03:30] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-object-wf.rs
[00:03:32] some tidy checks failed
[00:03:32] 
[00:03:32] 
[00:03:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:32] 
[00:03:32] 
[00:03:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:32] Build completed unsuccessfully in 0:00:45
[00:03:32] Build completed unsuccessfully in 0:00:45
[00:03:32] Makefile:67: recipe for target 'tidy' failed
[00:03:32] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004d4ad0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 02:25:19 UTC 2019
---
travis_time:end:01980d5e:start=1554258320147704325,finish=1554258320153575107,duration=5870782
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a1c7cf6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00002a72
travis_time:start:00002a72
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12697a19
$ dmesg | grep -i kill
