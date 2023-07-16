plain
travis_time:end:0f0ddda7:start=1552451371479680683,finish=1552451448956299056,duration=77476618373
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:21] tidy error: /checkout/src/test/ui/parser/doc-inside-trait-item.rs: missing trailing newline
[00:03:22] some tidy checks failed
[00:03:22] 
[00:03:22] 
[00:03:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:22] 
[00:03:22] 
[00:03:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:22] Build completed unsuccessfully in 0:00:47
[00:03:22] Build completed unsuccessfully in 0:00:47
[00:03:22] Makefile:68: recipe for target 'tidy' failed
[00:03:22] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003e61d1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 04:34:20 UTC 2019
---
travis_time:end:064afad9:start=1552451661684368928,finish=1552451661689762848,duration=5393920
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11965edf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1dcf7220
travis_time:start:1dcf7220
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:171e1360
$ dmesg | grep -i kill
