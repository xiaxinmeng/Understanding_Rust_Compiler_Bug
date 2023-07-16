plain
travis_time:end:14fe0160:start=1555142633657877826,finish=1555142720944683332,duration=87286805506
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:832: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:833: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:834: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:835: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:851: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1028: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1035: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1070: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1142: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1171: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1207: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1210: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1211: line longer than 100 chars
[00:03:15] tidy error: /checkout/src/libsyntax/feature_gate.rs:1256: line longer than 100 chars
[00:03:17] some tidy checks failed
[00:03:17] 
[00:03:17] 
[00:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:17] 
[00:03:17] 
[00:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:17] Build completed unsuccessfully in 0:00:47
[00:03:17] Build completed unsuccessfully in 0:00:47
[00:03:17] make: *** [tidy] Error 1
[00:03:17] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:268a02d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 13 08:08:47 UTC 2019
---
travis_time:end:19845ced:start=1555142928490000155,finish=1555142928495651162,duration=5651007
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05c0f340
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f5d5444
travis_time:start:0f5d5444
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24634bde
$ dmesg | grep -i kill
