plain
travis_time:end:128cd2d4:start=1555030896926319452,finish=1555030899043856226,duration=2117536774
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:22] tidy error: /checkout/src/librustc_codegen_ssa/mir/mod.rs:658: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:694: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:698: TODO is deprecated; use FIXME
[00:04:22] tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:1824: TODO is deprecated; use FIXME
[00:04:24] some tidy checks failed
[00:04:24] 
[00:04:24] 
[00:04:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:24] 
[00:04:24] 
[00:04:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:24] Build completed unsuccessfully in 0:00:47
[00:04:24] Build completed unsuccessfully in 0:00:47
[00:04:24] Makefile:67: recipe for target 'tidy' failed
[00:04:24] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19b3ab46
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 12 01:06:15 UTC 2019
---
travis_time:end:3e9219fa:start=1555031175943857515,finish=1555031175948701806,duration=4844291
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04b41768
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3ac6daf9
travis_time:start:3ac6daf9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11c207c0
$ dmesg | grep -i kill
