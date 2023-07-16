plain
travis_time:end:170d9c6c:start=1552411838060905419,finish=1552411840295555596,duration=2234650177
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:16] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:16] tidy error: binary checked into source: /checkout/src/test/ui/lint/auxiliary/stability-cfg2.rs
[00:04:17] thread 'main' panicked at 'File::open(file) failed on /checkout/src/test/ui/lint/auxiliary/stability-cfg2.rs with No such file or directory (os error 2)', src/tools/tidy/src/style.rs:104:12
[00:04:17] 
[00:04:17] 
[00:04:17] 
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] 
[00:04:17] 
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:45
[00:04:17] Build completed unsuccessfully in 0:00:45
[00:04:17] make: *** [tidy] Error 1
[00:04:17] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3254f86a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 17:35:07 UTC 2019
---
travis_time:end:0fc14e6c:start=1552412108361244349,finish=1552412108365520785,duration=4276436
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:011ed867
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0489d6dc
travis_time:start:0489d6dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12c371ee
$ dmesg | grep -i kill
