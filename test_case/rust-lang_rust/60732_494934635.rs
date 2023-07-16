plain
travis_time:end:04e9d938:start=1558550968753362227,finish=1558551058389099151,duration=89635736924
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:tidy
tidy check
[00:04:23] * 574 error codes
[00:04:23] * highest error code: E0729
[00:04:25] tidy error: /checkout/src/test/ui/enum-discriminant/feature-gate-arbitrary_enum_discriminant.rs:1: The file is already marked as gate test through its name, no need for a 'gate-test-arbitrary_enum_discriminant' comment
[00:04:28] some tidy checks failed
[00:04:28] 
[00:04:28] 
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] 
[00:04:28] 
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:01:15
[00:04:28] Build completed unsuccessfully in 0:01:15
[00:04:28] Makefile:67: recipe for target 'tidy' failed
[00:04:28] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:036d60b9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 22 18:55:36 UTC 2019
---
travis_time:end:02bedf80:start=1558551337636031143,finish=1558551337641642720,duration=5611577
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e6d0fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0352fa96
travis_time:start:0352fa96
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23151b13
$ dmesg | grep -i kill
