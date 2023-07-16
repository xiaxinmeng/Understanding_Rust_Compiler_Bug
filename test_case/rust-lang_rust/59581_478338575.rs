plain
travis_time:end:0f63693c:start=1554035412532917029,finish=1554035414647948673,duration=2115031644
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:tidy
tidy check
[00:04:07] * 569 error codes
[00:04:07] * highest error code: E0725
[00:04:07] tidy error: /checkout/src/libcore/cell.rs:721: mismatches the `issue` in corresponding lang feature
[00:04:08] some tidy checks failed
[00:04:08] 
[00:04:08] 
[00:04:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:08] 
[00:04:08] 
[00:04:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:08] Build completed unsuccessfully in 0:00:48
[00:04:08] Build completed unsuccessfully in 0:00:48
[00:04:08] Makefile:67: recipe for target 'tidy' failed
[00:04:08] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01b2eab8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 31 12:34:33 UTC 2019
---
travis_time:end:1ccbd674:start=1554035674146063371,finish=1554035674151204809,duration=5141438
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a68b948
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:173c62e4
travis_time:start:173c62e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e7f4016
$ dmesg | grep -i kill
