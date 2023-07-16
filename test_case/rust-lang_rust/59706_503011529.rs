plain
travis_time:end:31386b85:start=1560847177796658920,finish=1560847265683311028,duration=87886652108
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:515: line longer than 100 chars
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:516: line longer than 100 chars
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:527: line longer than 100 chars
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:528: line longer than 100 chars
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:537: line longer than 100 chars
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:541: line longer than 100 chars
[00:03:42] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:542: line longer than 100 chars
[00:03:47] some tidy checks failed
[00:03:47] 
[00:03:47] 
[00:03:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:47] 
[00:03:47] 
[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] Build completed unsuccessfully in 0:01:10
---
travis_time:end:19b99764:start=1560847503155211664,finish=1560847503159698621,duration=4486957
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:28273206
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:310209cc
travis_time:start:310209cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:022ed3a8
$ dmesg | grep -i kill
