plain
travis_time:end:0f6f98ee:start=1554230356407689251,finish=1554230464982127162,duration=108574437911
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:37] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/wf-trait-object-reverse-order.rs
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/wf-trait-object-no-duplicates.rs
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/wf-trait-object-no-sized.rs
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-extra-traits.rs
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs
[00:03:37] tidy error: binary checked into source: /checkout/src/test/ui/traits/trait-alias/trait-alias-object-wf.rs
[00:03:39] some tidy checks failed
[00:03:39] 
[00:03:39] 
[00:03:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:39] 
[00:03:39] 
[00:03:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:39] Build completed unsuccessfully in 0:00:46
[00:03:39] Build completed unsuccessfully in 0:00:46
[00:03:39] Makefile:67: recipe for target 'tidy' failed
[00:03:39] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:123cf046
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  2 18:44:55 UTC 2019
---
travis_time:end:1b01758c:start=1554230695874926133,finish=1554230695879913739,duration=4987606
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00f2bbe9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d0bca45
travis_time:start:0d0bca45
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04e20f78
$ dmesg | grep -i kill
