plain
travis_time:end:2fdb1671:start=1551800240217732239,finish=1551800241188561881,duration=970829642
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:05:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:23] tidy error: /checkout/src/librustc_target/spec/mipsisa64r6_unknown_linux_gnuabi64.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:05:23] tidy error: /checkout/src/librustc_target/spec/mipsisa64r6el_unknown_linux_gnuabi64.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:05:23] tidy error: /checkout/src/librustc_target/spec/mipsisa32r6el_unknown_linux_gnu.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:05:23] tidy error: /checkout/src/librustc_target/spec/mipsisa32r6_unknown_linux_gnu.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:05:24] some tidy checks failed
[00:05:24] 
[00:05:24] 
[00:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:24] 
[00:05:24] 
[00:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:24] Build completed unsuccessfully in 0:00:48
[00:05:24] Build completed unsuccessfully in 0:00:48
[00:05:24] Makefile:68: recipe for target 'tidy' failed
[00:05:24] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:161f384e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar  5 15:42:56 UTC 2019
---
travis_time:end:0e08b539:start=1551800577152276354,finish=1551800577156898210,duration=4621856
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:353947a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b2bc3c3
travis_time:start:0b2bc3c3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06779d08
$ dmesg | grep -i kill
