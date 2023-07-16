plain
travis_time:end:0e07edfc:start=1555042089252633692,finish=1555042091318747217,duration=2066113525
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:52] tidy error: /checkout/src/librustc_codegen_ssa/mir/mod.rs:658: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:694: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:698: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/librustc_codegen_llvm/debuginfo/metadata.rs:1824: TODO is deprecated; use FIXME
[00:03:54] some tidy checks failed
[00:03:54] 
[00:03:54] 
[00:03:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:54] 
[00:03:54] 
[00:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:54] Build completed unsuccessfully in 0:00:46
[00:03:54] Build completed unsuccessfully in 0:00:46
[00:03:54] Makefile:67: recipe for target 'tidy' failed
[00:03:54] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b596766
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 12 04:12:16 UTC 2019
---
travis_time:end:0cdb4488:start=1555042337737740424,finish=1555042337743025641,duration=5285217
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c7a4282
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0468ebc8
travis_time:start:0468ebc8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2ad963c8
$ dmesg | grep -i kill
