plain
travis_time:end:03786504:start=1555776638986520117,finish=1555776639777431561,duration=790911444
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:17] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:17] tidy error: /checkout/src/libsyntax/error_codes.rs:390: line longer than 100 chars
[00:04:18] tidy error: duplicate error code: 704
[00:04:18] tidy error: /checkout/src/libsyntax/error_codes.rs:366: E0704: r##"
[00:04:18] tidy error: /checkout/src/libsyntax/error_codes.rs:447:     E0704, // incorrect visibility restriction
[00:04:19] some tidy checks failed
[00:04:19] 
[00:04:19] 
[00:04:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:19] 
[00:04:19] 
[00:04:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:19] Build completed unsuccessfully in 0:00:46
[00:04:19] Build completed unsuccessfully in 0:00:46
[00:04:19] make: *** [tidy] Error 1
[00:04:19] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:168b9e22
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 16:15:10 UTC 2019
---
travis_time:end:0aadbb41:start=1555776911240484280,finish=1555776911245361971,duration=4877691
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:262f79c1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00888618
travis_time:start:00888618
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11f698c0
$ dmesg | grep -i kill
