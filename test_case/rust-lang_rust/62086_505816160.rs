plain
travis_time:end:14721520:start=1561543642449632663,finish=1561543644796921899,duration=2347289236
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:tidy
tidy check
[00:04:59] * 577 error codes
[00:04:59] * highest error code: E0732
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:747: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:846: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:929: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:1199: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:1213: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:1223: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:1233: malformed stability attribute: missing `feature` key
[00:04:59] tidy error: /checkout/src/libcore/macros.rs:1258: malformed stability attribute: missing `feature` key
[00:05:00] tidy error: /checkout/src/libcore/macros.rs contains #[test]; libcore tests must be placed inside `src/libcore/tests/`
[00:05:00] some tidy checks failed
[00:05:00] 
[00:05:00] 
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:05:00] 
[00:05:00] 
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:00] Build completed unsuccessfully in 0:01:13
---
travis_time:end:1addf6ac:start=1561543957375651179,finish=1561543957380540750,duration=4889571
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0da8066e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:063527d4
travis_time:start:063527d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23b68401
$ dmesg | grep -i kill
