plain
travis_time:end:1859ed04:start=1551555432092975734,finish=1551555433091701157,duration=998725423
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:55] 
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56]     Updating crates.io index
[00:02:08]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:09]   Downloaded getopts v0.2.17
[00:02:09]   Downloaded serde_json v1.0.33
[00:02:09]   Downloaded num_cpus v1.8.0
[00:02:09]   Downloaded toml v0.4.10
---
tidy check
[00:04:06] * 569 error codes
[00:04:06] * highest error code: E0724
[00:04:06] * 252 features
[00:04:07] invalid source: "git+https://github.com/gnzlbg/libtest#63ae76050ad3a2c81423f83d3bf4106ade630d4f"
[00:04:07] invalid source: "git+https://github.com/gnzlbg/libtest#63ae76050ad3a2c81423f83d3bf4106ade630d4f"
[00:04:07] some tidy checks failed
[00:04:07] 
[00:04:07] 
[00:04:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:07] 
[00:04:07] 
[00:04:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:07] Build completed unsuccessfully in 0:00:48
[00:04:07] Build completed unsuccessfully in 0:00:48
[00:04:07] make: *** [tidy] Error 1
[00:04:07] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06bbf39a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  2 19:41:31 UTC 2019
---
travis_time:end:0606b0ca:start=1551555692869335904,finish=1551555692874687767,duration=5351863
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:009bbee3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:207cdd4e
travis_time:start:207cdd4e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a968576
$ dmesg | grep -i kill
