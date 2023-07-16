plain
travis_time:end:11f30564:start=1549751305892799430,finish=1549751306856293444,duration=963494014
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:51] tidy error: /checkout/src/librustc/traits/query/normalize.rs:194: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/traits/project.rs:398: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/traits/structural_impls.rs:751: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/traits/structural_impls.rs:761: TODO is deprecated; use FIXME
[00:03:51] tidy error: /checkout/src/librustc/traits/structural_impls.rs:885: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/traits/structural_impls.rs:901: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/traits/structural_impls.rs:942: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/traits/structural_impls.rs:963: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/infer/combine.rs:359: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/subst.rs:132: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/subst.rs:435: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/fold.rs:46: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/fold.rs:47: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/fold.rs:203: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/fold.rs:433: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/fold.rs:495: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/fold.rs:758: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/structural_impls.rs:599: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/structural_impls.rs:824: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/structural_impls.rs:971: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/ty/structural_impls.rs:1176: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/mir/mod.rs:2627: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/mir/mod.rs:3118: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/mir/mod.rs:3285: TODO is deprecated; use FIXME
[00:03:51] tidy error: /checkout/src/librustc/mir/mod.rs:3286: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/mir/mod.rs:3303: line longer than 100 chars
[00:03:51] tidy error: /checkout/src/librustc/mir/mod.rs:3392: line longer than 100 chars
[00:03:52] some tidy checks failed
[00:03:52] 
[00:03:52] 
[00:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:52] 
[00:03:52] 
[00:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:52] Build completed unsuccessfully in 0:00:44
[00:03:52] Build completed unsuccessfully in 0:00:44
[00:03:52] make: *** [tidy] Error 1
[00:03:52] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ea87945
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 22:32:30 UTC 2019
---
travis_time:end:07946775:start=1549751551610346988,finish=1549751551614784069,duration=4437081
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06031398
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0006d81b
travis_time:start:0006d81b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:248c4f54
$ dmesg | grep -i kill
