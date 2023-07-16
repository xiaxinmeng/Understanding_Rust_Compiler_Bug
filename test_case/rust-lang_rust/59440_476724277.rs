plain
travis_time:end:269af8c8:start=1553616223351120493,finish=1553616224265450766,duration=914330273
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
###################################################################       94.2%
######################################################################## 100.0%
[00:02:04] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:04]     Updating crates.io index
[00:02:18]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:19]   Downloaded serde_json v1.0.33
[00:02:19]   Downloaded time v0.1.40
[00:02:19]   Downloaded cc v1.0.28
[00:02:19]   Downloaded libc v0.2.50
---

[00:04:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:09] tidy error: /checkout/src/libtest/lib.rs:25: line longer than 100 chars
[00:04:09] tidy error: /checkout/src/libtest/lib.rs:26: line longer than 100 chars
[00:04:09] tidy error: /checkout/src/libtest/lib.rs:27: line longer than 100 chars
[00:04:10] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#719256356b565a138e0005cabbc56153e5902dca"
[00:04:10] some tidy checks failed
[00:04:10] 
[00:04:10] 
[00:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:10] 
[00:04:10] 
[00:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:10] Build completed unsuccessfully in 0:00:45
[00:04:10] Build completed unsuccessfully in 0:00:45
[00:04:10] Makefile:67: recipe for target 'tidy' failed
[00:04:10] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06f10116
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 16:08:06 UTC 2019
---
travis_time:end:0b0f404b:start=1553616487211729932,finish=1553616487216499799,duration=4769867
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a3cb938
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05455b78
travis_time:start:05455b78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27afa830
$ dmesg | grep -i kill
