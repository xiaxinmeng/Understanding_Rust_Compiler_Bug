plain
travis_time:end:0653b3f8:start=1553827120828308087,finish=1553827123013924573,duration=2185616486
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:18] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:19] tidy error: /checkout/src/ci/docker/dist-various-2/build-wasi-toolchain.sh:5: line longer than 100 chars
[00:04:19] tidy error: /checkout/src/ci/docker/dist-various-2/build-wasi-toolchain.sh:6: line longer than 100 chars
[00:04:20] some tidy checks failed
[00:04:20] 
[00:04:20] 
[00:04:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:20] 
[00:04:20] 
[00:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:20] Build completed unsuccessfully in 0:00:46
[00:04:20] Build completed unsuccessfully in 0:00:46
[00:04:20] make: *** [tidy] Error 1
[00:04:20] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15670647
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 29 02:43:16 UTC 2019
---
travis_time:end:201b9e02:start=1553827397744973737,finish=1553827397752838945,duration=7865208
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0157a544
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d56d55c
travis_time:start:0d56d55c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:218f36b4
$ dmesg | grep -i kill
