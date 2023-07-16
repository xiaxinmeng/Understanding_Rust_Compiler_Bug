plain
travis_time:end:12459198:start=1551018757504139059,finish=1551018758446943958,duration=942804899
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:53] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:53] tidy error: /checkout/src/librustc/lint/internal.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:53] tidy error: /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:53] tidy error: /checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:53] tidy error: /checkout/src/test/ui-fulldeps/internal-lints/without_compile_flag.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:54] 
[00:03:54] 
[00:03:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:54] 
[00:03:54] 
[00:03:54] some tidy checks failed
[00:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:54] Build completed unsuccessfully in 0:00:46
[00:03:54] make: *** [tidy] Error 1
[00:03:54] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17e7e5f2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 24 14:36:43 UTC 2019
---
travis_time:end:0523bc70:start=1551019004418831307,finish=1551019004423442387,duration=4611080
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00bf6222
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:032ae90a
travis_time:start:032ae90a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d8f44ea
$ dmesg | grep -i kill
