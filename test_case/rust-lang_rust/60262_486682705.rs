plain
travis_time:end:0ad4eb2f:start=1556199498750369215,finish=1556199499524060060,duration=773690845
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:07] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:07] tidy error: /checkout/src/test/run-make-fulldeps/pgo-use/main.rs:13: tab character
[00:04:07] tidy error: /checkout/src/test/run-make-fulldeps/pgo-use/main.rs: leading newline
[00:04:09] some tidy checks failed
[00:04:09] 
[00:04:09] 
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:09] 
[00:04:09] 
[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:09] Build completed unsuccessfully in 0:00:45
[00:04:09] Build completed unsuccessfully in 0:00:45
[00:04:09] make: *** [tidy] Error 1
[00:04:09] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:351f6cbc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 13:42:39 UTC 2019
---
travis_time:end:001847a0:start=1556199760651260378,finish=1556199760655777291,duration=4516913
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07676568
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0028cf9c
travis_time:start:0028cf9c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04f6a439
$ dmesg | grep -i kill
