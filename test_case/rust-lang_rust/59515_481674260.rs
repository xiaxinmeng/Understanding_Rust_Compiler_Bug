plain
travis_time:end:0664e000:start=1554899170744759651,finish=1554899269918227712,duration=99173468061
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:49]   Downloaded chalk-engine v0.9.0
[00:02:49]   Downloaded phf_codegen v0.7.22
[00:02:49]   Downloaded regex-syntax v0.5.6
[00:02:49]   Downloaded fst v0.3.0
[00:02:49]   Downloaded measureme v0.2.0
[00:02:49]   Downloaded argon2rs v0.2.5
[00:02:49]   Downloaded tokio-threadpool v0.1.10
[00:02:49]   Downloaded byteorder v1.2.7
[00:02:49]   Downloaded aho-corasick v0.6.9
---

[00:03:36] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:36] tidy error: /checkout/src/librustc/util/profiling.rs:126: line longer than 100 chars
[00:03:37] Dependencies not on the whitelist:
[00:03:37] * measureme 
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:44
[00:03:37] Build completed unsuccessfully in 0:00:44
[00:03:37] Makefile:67: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23f37780
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 12:31:37 UTC 2019
---
travis_time:end:0d2bf160:start=1554899498312231660,finish=1554899498316966853,duration=4735193
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07056286
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07a6381a
travis_time:start:07a6381a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08fd9af0
$ dmesg | grep -i kill
