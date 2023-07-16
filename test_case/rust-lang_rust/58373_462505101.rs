plain
travis_time:end:0cfbd85a:start=1549918893896421766,finish=1549918970418710447,duration=76522288681
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:21] tidy error: binary checked into source: /checkout/src/libcore/core_arch_docs.md
[00:03:23] some tidy checks failed
[00:03:23] 
[00:03:23] 
[00:03:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:23] 
[00:03:23] 
[00:03:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:23] Build completed unsuccessfully in 0:00:45
[00:03:23] Build completed unsuccessfully in 0:00:45
[00:03:23] Makefile:68: recipe for target 'tidy' failed
[00:03:23] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09f9c28a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 21:06:22 UTC 2019
---
travis_time:end:276fbfe0:start=1549919183602131603,finish=1549919183606817845,duration=4686242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0536fdf6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c79f650
travis_time:start:0c79f650
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27f3ae54
$ dmesg | grep -i kill
