plain
travis_time:end:03603c51:start=1549557147641131588,finish=1549557294147151753,duration=146506020165
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:07] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:07] tidy error: binary checked into source: /checkout/src/test/run-make/wasm-stringify-ints-small/foo.rs
[00:04:07] tidy error: /checkout/src/libcore/fmt/num.rs:265: line longer than 100 chars
[00:04:09] some tidy checks failed
[00:04:09] 
[00:04:09] 
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:09] 
[00:04:09] 
[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:09] Build completed unsuccessfully in 0:00:47
[00:04:09] Build completed unsuccessfully in 0:00:47
[00:04:09] Makefile:68: recipe for target 'tidy' failed
[00:04:09] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ec421a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 16:39:12 UTC 2019
---
travis_time:end:364154c7:start=1549557553634501702,finish=1549557553639193304,duration=4691602
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:057482e5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:352ef240
travis_time:start:352ef240
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2fa96ad0
$ dmesg | grep -i kill
