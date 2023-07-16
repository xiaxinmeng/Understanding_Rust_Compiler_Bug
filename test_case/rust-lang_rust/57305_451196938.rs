plain
travis_time:end:077f5535:start=1546531946205272076,finish=1546532000532276263,duration=54327004187
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:04:14] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:14] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:843: line longer than 100 chars
[00:04:15] some tidy checks failed
[00:04:15] 
[00:04:15] 
[00:04:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:15] 
[00:04:15] 
[00:04:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:15] Build completed unsuccessfully in 0:00:55
[00:04:15] Build completed unsuccessfully in 0:00:55
[00:04:15] Makefile:79: recipe for target 'tidy' failed
[00:04:15] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a7f0f30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 16:17:45 UTC 2019
---
travis_time:end:01a90fa0:start=1546532265682732826,finish=1546532265687536384,duration=4803558
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05b85446
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e907d11
travis_time:start:0e907d11
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:013308c3
$ dmesg | grep -i kill
