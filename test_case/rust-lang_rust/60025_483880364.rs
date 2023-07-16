plain
travis_time:end:11dac082:start=1555456431325323025,finish=1555456432111206565,duration=785883540
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:49] tidy error: duplicate error code: 424
[00:03:49] tidy error: /checkout/src/librustc_resolve/diagnostics.rs:90:             debug!("smart_resolve_path_fragment: E0424, source={:?}", source);
[00:03:49] tidy error: /checkout/src/librustc_resolve/diagnostics.rs:92:             __diagnostic_used!(E0424);
[00:03:49] tidy error: /checkout/src/librustc_resolve/diagnostics.rs:93:             err.code(DiagnosticId::Error("E0424".into()));
[00:03:49] tidy error: duplicate error code: 411
[00:03:49] tidy error: /checkout/src/librustc_resolve/diagnostics.rs:83:             __diagnostic_used!(E0411);
[00:03:49] tidy error: /checkout/src/librustc_resolve/diagnostics.rs:84:             err.code(DiagnosticId::Error("E0411".into()));
[00:03:50] some tidy checks failed
[00:03:50] 
[00:03:50] 
[00:03:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:50] 
[00:03:50] 
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:50] Build completed unsuccessfully in 0:00:47
[00:03:50] Build completed unsuccessfully in 0:00:47
[00:03:50] make: *** [tidy] Error 1
[00:03:50] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bf8f60c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 16 23:17:54 UTC 2019
---
travis_time:end:02bae0c7:start=1555456675801778419,finish=1555456675807360129,duration=5581710
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:096b8c40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21b4f3fc
travis_time:start:21b4f3fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3b728df4
$ dmesg | grep -i kill
