plain
travis_time:end:0c67c584:start=1558396729181825675,finish=1558396820334010528,duration=91152184853
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:28] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:29] tidy error: /checkout/src/librustdoc/docfs.rs:50: TODO is deprecated; use FIXME
[00:04:29] tidy error: /checkout/src/librustdoc/html/render.rs:822: line longer than 100 chars
[00:04:29] tidy error: /checkout/src/librustdoc/html/render.rs:910: line longer than 100 chars
[00:04:29] tidy error: /checkout/src/librustdoc/html/render.rs:916: line longer than 100 chars
[00:04:29] tidy error: /checkout/src/librustdoc/html/render.rs:1134: line longer than 100 chars
[00:04:29] tidy error: /checkout/src/librustdoc/html/render.rs:1276: line longer than 100 chars
[00:04:34] some tidy checks failed
[00:04:34] 
[00:04:34] 
[00:04:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:34] 
[00:04:34] 
[00:04:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:34] Build completed unsuccessfully in 0:01:18
[00:04:34] Build completed unsuccessfully in 0:01:18
[00:04:34] Makefile:67: recipe for target 'tidy' failed
[00:04:34] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ce6bd70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 21 00:05:03 UTC 2019
---
travis_time:end:0153988e:start=1558397104583964642,finish=1558397104590241530,duration=6276888
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02bd203f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0021ecb5
travis_time:start:0021ecb5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:104b0fe0
$ dmesg | grep -i kill
