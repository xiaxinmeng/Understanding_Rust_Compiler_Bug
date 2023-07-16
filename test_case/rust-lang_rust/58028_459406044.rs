plain
travis_time:end:005ff994:start=1548948588015930186,finish=1548948588934019490,duration=918089304
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:20] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:21] tidy error: /checkout/src/librustdoc/html/layout.rs:184: line longer than 100 chars
[00:04:22] some tidy checks failed
[00:04:22] 
[00:04:22] 
[00:04:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:22] 
[00:04:22] 
[00:04:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:22] Build completed unsuccessfully in 0:00:45
[00:04:22] Build completed unsuccessfully in 0:00:45
[00:04:22] Makefile:68: recipe for target 'tidy' failed
[00:04:22] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07bedf8e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 15:34:22 UTC 2019
---
travis_time:end:02185374:start=1548948862706047828,finish=1548948862710578370,duration=4530542
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03872514
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:040e9fa0
travis_time:start:040e9fa0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fdc5dc7
$ dmesg | grep -i kill
