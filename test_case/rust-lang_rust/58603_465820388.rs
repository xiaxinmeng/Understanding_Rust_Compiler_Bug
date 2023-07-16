plain
travis_time:end:21f4e914:start=1550710596010094621,finish=1550710596807374847,duration=797280226
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:08:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:4: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:7: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:10: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:14: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:15: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:16: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:17: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:18: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:19: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:20: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:24: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:25: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:26: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:27: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:28: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:29: tab character
[00:08:42] tidy error: /checkout/src/doc/jump-version.js:30: tab character
[00:08:44] some tidy checks failed
[00:08:44] 
[00:08:44] 
[00:08:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:08:44] 
[00:08:44] 
[00:08:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:08:44] Build completed unsuccessfully in 0:00:47
[00:08:44] Build completed unsuccessfully in 0:00:47
[00:08:44] Makefile:68: recipe for target 'tidy' failed
[00:08:44] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04fc5d6b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 21 01:05:32 UTC 2019
---
travis_time:end:0e039fa7:start=1550711133951371622,finish=1550711133955702601,duration=4330979
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26ee5e78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24117561
travis_time:start:24117561
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01fbaee4
$ dmesg | grep -i kill
