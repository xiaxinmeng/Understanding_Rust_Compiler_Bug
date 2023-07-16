plain
travis_time:end:0689cc2c:start=1552422112752052025,finish=1552422184770303553,duration=72018251528
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:02] tidy error: /checkout/src/test/run-make-fulldeps/redundant-libs/foo.c:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:04:02] tidy error: /checkout/src/test/run-make-fulldeps/redundant-libs/main.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:04:02] tidy error: /checkout/src/test/run-make-fulldeps/redundant-libs/baz.c:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:04:02] tidy error: /checkout/src/test/run-make-fulldeps/redundant-libs/bar.c:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:04:03] some tidy checks failed
[00:04:03] 
[00:04:03] 
[00:04:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:03] 
[00:04:03] 
[00:04:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:03] Build completed unsuccessfully in 0:00:51
[00:04:03] Build completed unsuccessfully in 0:00:51
[00:04:03] make: *** [tidy] Error 1
[00:04:03] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00359757
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 20:27:19 UTC 2019
---
travis_time:end:324e1ac9:start=1552422440122087040,finish=1552422440127170794,duration=5083754
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14f03ee3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e9751c
travis_time:start:01e9751c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a0656b6
$ dmesg | grep -i kill
