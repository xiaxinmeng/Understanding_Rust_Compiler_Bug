plain
travis_time:end:040a8ea4:start=1551552090735629449,finish=1551552091684831951,duration=949202502
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
###########################################                               60.6%
######################################################################## 100.0%
[00:02:10] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:10]     Updating crates.io index
[00:02:22]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:23]   Downloaded filetime v0.2.4
[00:02:23]   Downloaded serde_json v1.0.33
[00:02:23]   Downloaded serde_derive v1.0.81
[00:02:23]   Downloaded libc v0.2.46
---
tidy check
[00:04:17] * 569 error codes
[00:04:17] * highest error code: E0724
[00:04:17] * 252 features
[00:04:18] invalid source: "git+https://github.com/gnzlbg/libtest#63ae76050ad3a2c81423f83d3bf4106ade630d4f"
[00:04:18] invalid source: "git+https://github.com/gnzlbg/libtest#63ae76050ad3a2c81423f83d3bf4106ade630d4f"
[00:04:18] some tidy checks failed
[00:04:18] 
[00:04:18] 
[00:04:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:18] 
[00:04:18] 
[00:04:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:18] Build completed unsuccessfully in 0:00:47
[00:04:18] Build completed unsuccessfully in 0:00:47
[00:04:18] Makefile:68: recipe for target 'tidy' failed
[00:04:18] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e9d2d95
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  2 18:46:01 UTC 2019
---
travis_time:end:21f10141:start=1551552362055433616,finish=1551552362060201458,duration=4767842
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09370235
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:011c44fc
travis_time:start:011c44fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:070ab3de
$ dmesg | grep -i kill
