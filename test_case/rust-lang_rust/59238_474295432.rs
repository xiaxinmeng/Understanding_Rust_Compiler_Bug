plain
travis_time:end:0b90254d:start=1552990673344889172,finish=1552990766072404099,duration=92727514927
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:9: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:10: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:12: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:26: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:28: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:36: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:38: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/test/ui/simd-ffi.rs:40: line longer than 100 chars
[00:03:11] some tidy checks failed
[00:03:11] 
[00:03:11] 
[00:03:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:11] 
[00:03:11] 
[00:03:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:11] Build completed unsuccessfully in 0:00:44
[00:03:11] Build completed unsuccessfully in 0:00:44
[00:03:11] Makefile:67: recipe for target 'tidy' failed
[00:03:11] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20040f54
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 10:22:46 UTC 2019
---
travis_time:end:0fdc4ba0:start=1552990967256207985,finish=1552990967261170962,duration=4962977
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:266b0cb7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ac5a240
travis_time:start:2ac5a240
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03a7c658
$ dmesg | grep -i kill
